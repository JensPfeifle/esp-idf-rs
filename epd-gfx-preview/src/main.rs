#![deny(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use chrono::{Timelike, Utc};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::GrayColor;
use epd_gfx::openmeteo;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use preview::PreviewDisplay;
use reqwest;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub mod preview;

const WINDOW_WIDTH: u32 = 540;
const WINDOW_HEIGHT: u32 = 960;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    let mut close_requested = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    close_requested = true;
                }
                WindowEvent::Resized(size) => {
                    // Resize the window
                    pixels.resize_surface(size.width, size.height);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match virtual_code {
                    VirtualKeyCode::Escape => {
                        close_requested = true;
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                if close_requested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::RedrawRequested(_) => {
                let update_result = world.update();
                if let Err(result) = update_result {
                    error!("world update failed: {:?}", result);
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                world.draw(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::RedrawEventsCleared => {
                *control_flow = ControlFlow::Wait;
            }
            _ => (),
        }
    });
}

pub fn fetch_current_weather(
    config: openmeteo::OpenMeteoConfig,
) -> Result<openmeteo::OpenMeteoData> {
    let client = reqwest::blocking::Client::new();
    let base = "https://api.open-meteo.com/v1/forecast?".to_owned();
    let query_params = config.into_tuples();
    let res = client.get(base).query(&query_params).send()?;
    let body = res.bytes()?;
    println!("{body:?}");
    let data: openmeteo::OpenMeteoData =
        serde_json::from_slice(&body).context("Unable to decode response")?;
    println!("{data:?}");
    // FIXME: Parse/handle error

    Ok(data)
}

struct World {
    display: PreviewDisplay,
    data: Option<openmeteo::OpenMeteoData>,
}

impl World {
    fn new() -> Self {
        Self {
            display: PreviewDisplay::new(),
            data: None,
        }
    }

    fn update(&mut self) -> Result<()> {
        let ettlingen = openmeteo::Location {
            lat: 48.93,
            lon: 8.4,
        };

        self.display.clear(Gray4::WHITE)?;

        let now = Utc::now();
        let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second(),);
        let date = now.date().format("%Y-%m-%d").to_string();
        epd_gfx::draw_grid(10, 10, 0xD, &mut self.display)?;
        epd_gfx::draw_grid(50, 50, 0xA, &mut self.display)?;
        epd_gfx::draw_header(&time, &date, &mut self.display)?;

        let params = openmeteo::OpenMeteoConfig::new(ettlingen);
        match fetch_current_weather(params) {
            Ok(data) => {
                self.data = Some(data);
            }
            Err(e) => return Err(e.context("Fetching weather data failed")),
        }

        if let Some(ref weather_data) = self.data {
            if let Some(ref current_weather) = weather_data.current_weather {
                epd_gfx::draw_current_weather(
                    &current_weather.weathercode,
                    current_weather.temperature,
                    &mut self.display,
                )?;
            }
        }

        Ok(())
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let screen_x = (i % 540 as usize) as i32;
            let screen_y = (i / 540 as usize) as i32;

            if let Some(shade) = self.display.get_pixel(screen_x, screen_y) {
                // Scale range from 4 bits to 1 byte (0-255).
                let rgba = [shade * 15, shade * 15, shade * 15, 0xff];
                pixel.copy_from_slice(&rgba);
            }
        }
    }
}
