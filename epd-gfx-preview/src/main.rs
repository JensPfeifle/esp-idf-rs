#![deny(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::Result;
use embedded_graphics::prelude::*;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use preview::PreviewDisplay;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub mod preview;
pub mod weather;

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
                world.update();
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

struct World {
    display: PreviewDisplay,
}

impl World {
    fn new() -> Self {
        Self {
            display: PreviewDisplay::new(),
        }
    }

    fn update(&mut self) -> Result<()> {
        self.icons()?;
        Ok(())
    }

    fn icons(&mut self) -> Result<()> {
        let x1 = 120;
        let x2 = 400;

        let dy = 180;
        let mut y = 100;

        use epd_gfx::icons::*;

        ClearDay {
            pos: Point::new(x1, y),
        }
        .draw(&mut self.display)?;

        ClearNight {
            pos: Point::new(x2, y),
        }
        .draw(&mut self.display)?;

        y += dy;
        PartlyCloudyDay {
            pos: Point::new(x1, y),
        }
        .draw(&mut self.display)?;
        PartlyCloudyNight {
            pos: Point::new(x2, y),
        }
        .draw(&mut self.display)?;

        y += dy;
        Wind {
            pos: Point::new(x1, y),
        }
        .draw(&mut self.display)?;
        Rain {
            pos: Point::new(x2, y),
        }
        .draw(&mut self.display)?;

        y += dy;
        Snow {
            pos: Point::new(x1, y),
        }
        .draw(&mut self.display)?;
        Thunderstorm {
            pos: Point::new(x2, y),
        }
        .draw(&mut self.display)?;

        y += dy;
        Fog {
            pos: Point::new(x1, y),
        }
        .draw(&mut self.display)?;
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
