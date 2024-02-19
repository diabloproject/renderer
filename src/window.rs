use std::marker::PhantomData;
use std::ops::Deref;
use std::process::exit;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::renderable::{Color, PointInfo, Renderable};
use crate::{HEIGHT, WIDTH};

macro_rules! trust_me_bro {
    ($e: expr) => {unsafe { $e }};
}

pub struct Window<'t> {
    w: usize,
    h: usize,
    renderable: &'t dyn Renderable,
}


impl<'t> Window<'t> {
    pub fn new(w: usize, h: usize, renderable: &'t impl Renderable) -> Self {

        Self { w, h, renderable }
    }

    pub fn boot(&mut self) {}

    pub fn render(&self, t: f64, dt: f64) -> Vec<Color> {
        let mut data: Vec<Color> = Vec::with_capacity(self.w * self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                data.push(self.renderable.render(PointInfo {
                    x: x as f64,
                    y: y as f64,
                    dx: 1.0,
                    dy: 1.0,
                    t,
                    dt,
                }))
            }
        }
        data
    }

    pub fn run(self) {

    }
}