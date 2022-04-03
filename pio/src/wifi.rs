use anyhow::{bail, Result};
use embedded_svc::http::client::*;
use embedded_svc::ipv4;
use embedded_svc::ping::Ping;
use embedded_svc::wifi::*;
use epd_gfx::openmeteo;
use esp_idf_svc::http::client::{EspHttpClient, EspHttpResponse};
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;

use std::sync::Arc;
use std::time::Duration;
const SSID: &str = env!("ESP32_WIFI_SSID");
const PASS: &str = env!("ESP32_WIFI_PASS");

pub fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    println!("Wifi created, about to scan");

    let ap_printlns = wifi.scan()?;

    let ours = ap_printlns.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        println!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        println!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    println!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| {
            anyhow::anyhow!(
                "Unexpected Wifi status: {:?}. Please double-check the passsword.",
                e
            )
        })?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        println!("Wifi connected");

        ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

pub struct WeatherApi {
    client: Box<EspHttpClient>,
    config: openmeteo::OpenMeteoConfig,
    url: String,
}

impl WeatherApi {
    pub fn new(config: openmeteo::OpenMeteoConfig) -> Result<Self> {
        let client = EspHttpClient::new_default()?;
        Ok(Self {
            client: Box::new(client),
            url: Self::build_url(&config),
            config,
        })
    }

    fn build_url(config: &openmeteo::OpenMeteoConfig) -> String {
        let base = "http://api.open-meteo.com/v1/forecast".to_owned();
        let query_params = config.into_tuples();
        // FIXME: Urlencode?
        let params = query_params
            .iter()
            .map(|(q, v)| format!("{q}={v}"))
            .collect::<Vec<String>>();
        let url = base + "?" + &params.join("&");
        return url;
    }

    pub fn get(&mut self) -> Result<EspHttpResponse> {
        let req = self.client.get(self.url.clone())?;
        println!("Fetching from {}", self.url);
        let resp = req.submit()?;
        Ok(resp)
    }
}
fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
    println!("About to do some pings for {:?}", ip_settings);

    let ping_summary =
        ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    println!("Pinging done");

    Ok(())
}
