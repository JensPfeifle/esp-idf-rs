use anyhow::{bail, Result};
use embedded_svc::http::client::*;
use embedded_svc::http::*;
use embedded_svc::ipv4;
use embedded_svc::ping::Ping;
use embedded_svc::wifi::*;
use esp_idf_svc::http::client::{EspHttpClient, EspHttpResponse};
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::wifi::*;
use log::*;

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
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

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
    url: String,
}

impl WeatherApi {
    pub fn new() -> Result<Self> {
        let client = EspHttpClient::new_default()?;
        Ok(Self {
            client: Box::new(client),
            url: "https://api.brightsky.dev/current_weather?lat=52&lon=7.6".to_owned(),
        })
    }

    pub fn get(&mut self) -> Result<EspHttpResponse> {
        let req = self.client.get(self.url.clone())?;
        let resp = req.submit()?;
        Ok(resp)
    }
    pub fn read(&mut self) -> Result<()> {
        let req = self.client.get(String::from("http://google.com"))?;
        let resp = req.submit()?;
        let bytes: Result<Vec<_>, _> = embedded_svc::io::Bytes::<_, 64>::new(resp.reader())
            .take(3000)
            .collect();

        let bytes = bytes?;

        //let conf: wifi::Configuration = serde_json::from_slice(&bytes)?;

        Ok(())
    }
}
fn ping(ip_settings: &ipv4::ClientSettings) -> Result<()> {
    info!("About to do some pings for {:?}", ip_settings);

    let ping_summary =
        ping::EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    info!("Pinging done");

    Ok(())
}
