#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

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

const FB_SIZE: usize = WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize / 2;

struct World {
    //  4 bits per pixel, 16 grayscale shades
    // `0xF` (white) through `0x8` (median gray) til `0x0`
    fb: [u8; FB_SIZE],
}

impl World {
    fn new() -> Self {
        Self {
            fb: [0xFF; FB_SIZE],
        }
    }

    fn update(&mut self) {
        epd_gfx::set_all(&mut self.fb, 0xFF);
        epd_gfx::font::draw_text(&mut self.fb, 0, 0, "Hello from RustType!", 32);
        epd_gfx::fill_rect(&mut self.fb, 50, 50, 400, 250, 0x0);
        epd_gfx::draw_line(&mut self.fb, 0, 0, 400, 200, 0xF);
        epd_gfx::draw_line(&mut self.fb, 0, 0, 539, 959, 0x0);

        epd_gfx::fill_circle(&mut self.fb, 250, 250, 30, 0x8);
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let screen_x = (i % 540 as usize) as u32;
            let screen_y = (i / 540 as usize) as u32;

            let (fb_x, fb_y) = epd_gfx::to_landscape(screen_x, screen_y).unwrap();
            let fb_index = ((fb_y * 960 + fb_x) / 2) as usize;
            let (left, right) = epd_gfx::split_byte(self.fb[fb_index]);
            let (left, right) = epd_gfx::split_byte(self.fb[fb_index]);

            let shade = {
                if fb_x % 2 == 0 {
                    right
                } else {
                    left
                }
            };

            // Scale range from 4 bits to 1 byte (0-255).
            let rgba = [shade * 15, shade * 15, shade * 15, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }
}
