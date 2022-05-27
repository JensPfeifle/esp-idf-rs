use anyhow::Result;

use core::time::Duration;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;
use esp_idf_svc::timer::EspTimerService;
use esp_idf_svc::timer::*;
use std::thread;

pub fn schedule_timer() -> Result<EspTimer> {
    thread::sleep(Duration::from_secs(3));

    println!("About to schedule a periodic timer.");
    let mut periodic_timer = EspTimerService::new()?.timer(move || {
        println!("Tick from periodic timer");
    })?;

    periodic_timer.every(Duration::from_secs(60))?;

    Ok(periodic_timer)
}
