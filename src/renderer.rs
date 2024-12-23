use crate::renderable::{Color, PointInfo, Renderable};
use rayon::prelude::*;
use std::fmt::Debug;
use std::time::Duration;

macro_rules! trust_me_bro {
    ($e: expr) => {
        unsafe { $e }
    };
}

pub struct Renderer<R: Renderable + ?Sized + std::panic::RefUnwindSafe + Sync> {
    w: usize,
    h: usize,
    renderable: Box<R>,
}

impl<R: Renderable + ?Sized + std::panic::RefUnwindSafe + Sync> Renderer<R> {
    pub fn new(w: usize, h: usize, renderable: Box<R>) -> Self {
        Self { w, h, renderable }
    }

    pub fn render(&self, t: Duration, dt: Duration, initial: (f64, f64)) -> Vec<Color> {
        let mut positions: Vec<(f64, f64)> = Vec::with_capacity(self.w * self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                positions.push((x as f64 + initial.0, y as f64 + initial.1))
            }
        }
        println!("{:?}", dt.as_millis());
        let data = positions
            .into_par_iter()
            .map(|(x, y)| {
                (&self.renderable).render(PointInfo {
                    x: x as f64,
                    y: y as f64,
                    // dx: 1.0,
                    // dy: 1.0,
                    t,
                    // dt,
                })
            })
            .collect::<Vec<_>>();
        data
    }
}
