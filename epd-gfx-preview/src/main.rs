#![deny(clippy::all)]
#![forbid(unsafe_code)]
use anyhow::{Context, Error, Result};
use bytes::Bytes;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Gray4;
use embedded_graphics::prelude::GrayColor;
use epd_gfx::openmeteo;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use preview::PreviewDisplay;
use ureq;
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

pub fn fetch_data(url: &str) -> Result<Bytes> {
    let body: String = ureq::get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    return Ok(Bytes::from(body));
}

pub fn fetch_current_weather(
    config: openmeteo::OpenMeteoConfig,
) -> Result<openmeteo::OpenMeteoData> {
    let url = openmeteo::build_url(&config);
    let data = fetch_data(&url)?;
    let data: openmeteo::OpenMeteoData =
        serde_json::from_slice(&data).context("Unable to decode response")?;
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

        epd_gfx::draw_grid(10, 10, 0xD, &mut self.display)?;
        epd_gfx::draw_grid(50, 50, 0xA, &mut self.display)?;
        //epd_gfx::draw_icon_test_page(200, &mut self.display)?;

        let params = openmeteo::OpenMeteoConfig::new(ettlingen);
        match fetch_current_weather(params) {
            Ok(data) => {
                self.data = Some(data);
            }
            Err(e) => return Err(e.context("Fetching weather data failed")),
        }

        if let Some(ref d) = self.data {
            epd_gfx::draw(d, &mut self.display)?;
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
