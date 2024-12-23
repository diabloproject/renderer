mod font;
mod renderable;
mod renderables;
mod renderer;

use crate::renderable::*;
use crate::renderables::downsample::downsample;
use crate::renderables::image_render::*;
use crate::renderables::overlay::overlay;
use crate::renderables::pixelate::*;
use crate::renderer::Renderer;

use image::GenericImageView;
use pixels::{Pixels, SurfaceTexture};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::renderables::font_render::text_opentype;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Copy, Clone, Debug)]
pub struct SizeInfo {
    pub width: u32,
    pub height: u32,
}

pub struct Timer {
    first: Instant,
    last: Instant,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            first: Instant::now(),
            last: Instant::now(),
        };
    }
    pub fn elapse(&mut self) -> (Duration, Duration) {
        let total = self.first.elapsed();
        let current = self.last.elapsed();
        self.last = std::time::Instant::now();
        return (total, current);
    }
}

fn create_window<T>(event_loop: &EventLoop<T>, size_info: SizeInfo) -> Window {
    let size = LogicalSize::new(size_info.width as f64, size_info.height as f64);
    WindowBuilder::new()
        .with_title("Hello Pixels")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

fn create_pixels(window: &Window, size_info: SizeInfo) -> Pixels {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(size_info.width, size_info.height, surface_texture).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub struct NotEnoughArgumentsError;

impl Display for NotEnoughArgumentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Usage: cargo run <image_path>\n")?;
        Ok(())
    }
}

impl std::error::Error for NotEnoughArgumentsError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err(Box::new(NotEnoughArgumentsError));
    }
    let image_path: &str = &args[1];

    let im = image::open(image_path)?.to_rgba8();
    let size_info = SizeInfo {
        width: im.width() / 4,
        height: im.height() / 4,
    };

    let image_loader = load_image::new(LoadImageParams {
        path: Some(image_path),
        oob_clr: [255; 4].into(),
    })?;
    let f = Box::new(overlay {
        background: downsample {
            factor: 4.,
            child: image_loader,
        },
        foreground: |p: PointInfo| {
            let cv = (p.x + p.t.as_millis() as f64) % 256.;
            return [cv.round() as u8; 4].into();
            if ((p.x / 16.).round() as i32) % 2 == 0 {
                [0; 4].into()
            } else {
                [192; 4].into()
            }
        },
    });

    let renderer: Renderer<dyn Renderable + std::panic::RefUnwindSafe + Sync> =
        Renderer::new(size_info.width as usize, size_info.height as usize, f);
    let event_loop = EventLoop::new();
    let window = create_window(&event_loop, size_info);
    let mut pixels = create_pixels(&window, size_info);
    let mut timer = Timer::new();
    let mut input = WinitInputHelper::new();
    let mut drag_delta = (0., 0.);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let time = timer.elapse();
            println!("{:?}", time.1);
            let data = renderer
                .render(time.0, time.1, drag_delta)
                .into_iter()
                .map(|x| -> [u8; 4] { x.into() })
                .collect::<Vec<_>>()
                .concat();
            pixels.frame_mut().copy_from_slice(data.as_slice());
            if let Err(_err) = pixels.render() {
                *control_flow = ControlFlow::ExitWithCode(-1);
            }
            window.request_redraw();
        }
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.mouse_held(0) {
                let d = input.mouse_diff();
                // macos virtual resolution...
                drag_delta.0 -= (d.0 * 0.5) as f64;
                drag_delta.1 -= (d.1 * 0.5) as f64;
                window.request_redraw()
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                window.request_redraw()
            }

            // Update internal state and request a redraw
            // window.request_redraw();
        }
        // window.request_redraw();
    });
}
