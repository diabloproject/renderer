
mod window;
mod renderable;


use std::process::exit;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use renderable::*;
use crate::window::Window;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;


pub struct Timer {
    first: std::time::Instant,
    last: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            first: std::time::Instant::now(),
            last: std::time::Instant::now()
        }
    }
    pub fn elapse(&mut self) -> (f64, f64) {
        let total = self.first.elapsed();
        let current = self.last.elapsed();
        self.last = std::time::Instant::now();
        return (total.as_secs_f64(), current.as_secs_f64())
    }
}


struct nn_scale<R: Renderable + Sized + 'static> { pub factor: usize, pub child: R, pub lx: f64, pub ty: f64 }
impl<R: Renderable + Sized + 'static> Renderable for nn_scale<R> {
    fn render(&self, p: PointInfo) -> Color {
        self.child.render(PointInfo {
            x: ((p.x + self.lx) / self.factor as f64).round(),
            y: ((p.y + self.ty) / self.factor as f64).round(),
            t: p.t,
            dx: p.dx * self.factor as f64,
            dy: p.dy * self.factor as f64,
            dt: p.dt,
        })
    }
}

fn make_grid(p: PointInfo) -> Color {
    /// grid size is inf x inf with dx = 1, so
    /// lets scale our grid to dx = 4
    if ((p.x.round() + p.y.round()) as usize) % 2 == 0 {
        return [255; 4]
    }
    [0; 4]
}

static f: nn_scale<fn(PointInfo) -> Color> = nn_scale { factor: 8, lx: 4., ty: 4., child: make_grid };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = Window::new(320, 240, &f);

    let event_loop = EventLoop::new();
    let physical_window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = physical_window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &physical_window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut timer = Timer::new();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let time = timer.elapse();
            let data = window.render(time.0, time.1).concat();
            pixels.frame_mut().copy_from_slice(data.as_slice());
            if let Err(err) = pixels.render() {
                exit(-1);
            }
        }
        physical_window.request_redraw();
    });
    Ok(())
}