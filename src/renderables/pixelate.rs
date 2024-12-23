use crate::renderable::{Color, PointInfo, Renderable};

#[allow(non_camel_case_types)]
pub struct pixelate<T: Renderable> {
    pub pixel_size: u32,
    pub child: T,
}

fn compute_ceil(p1: f64, p2: u32) -> u32 {
    (p1 / p2 as f64).round() as u32 * p2
}

impl<T: Renderable> Renderable for pixelate<T> {
    fn render(&self, p: PointInfo) -> Color {
        let x = compute_ceil(p.x, self.pixel_size);
        let y = compute_ceil(p.y, self.pixel_size);
        let mut c: [u32; 4] = [0, 0, 0, 0];
        for ix in 0..self.pixel_size {
            let x = (ix + x) as f64;
            for iy in 0..self.pixel_size {
                let y = (iy + y) as f64;
                let t: [u8; 4] = self.child.render(PointInfo { x, y, ..p }).into();
                c[0] += t[0] as u32;
                c[1] += t[1] as u32;
                c[2] += t[2] as u32;
                c[3] += t[3] as u32;
            }
        }

        let v = self.pixel_size.pow(2);

        [
            (c[0] / v) as u8,
            (c[1] / v) as u8,
            (c[2] / v) as u8,
            (c[3] / v) as u8,
        ]
        .into()
    }
}
