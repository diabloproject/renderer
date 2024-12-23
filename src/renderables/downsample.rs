use crate::renderable::{Color, PointInfo, Renderable};

#[allow(non_camel_case_types)]
pub struct downsample<T: Renderable> {
    pub factor: f64,
    pub child: T
}


fn relu(x: f64) -> f64 {
    if x < 0. {
        0.
    } else {
        x
    }
}

impl<T: Renderable> Renderable for downsample<T> {
    fn render(&self, p: PointInfo) -> Color {
        // 3.3 * 1. | 4.3 * 1. | 5.3 * 1. <-- 3.
        // 3.3 * .5 | 4.3 * 1. | 5.3 * .5 <-- 2.

        let point_count = self.factor.ceil() as u32;
        let mut c: [f64; 4] = [0.; 4];
        for ix in 0..point_count {
            for iy in 0..point_count {
                let (ix, iy) = (ix as f64, iy as f64);
                let distance: (f64, f64) = (ix - point_count as f64/2., iy - point_count as f64/2.);
                let adjusted: (f64, f64) = (p.x * self.factor + distance.0, p.y * self.factor + distance.1);
                let weight = (1. - relu(distance.0 - self.factor / 2.)) * (1. - relu(distance.1 - self.factor / 2.));
                let result: [u8; 4] = self.child.render(PointInfo {
                    x: adjusted.0,
                    y: adjusted.1,
                    ..p
                }).into();
                for i in 0..4 {
                    c[i] += result[i] as f64 * weight
                }
            }
        }
        for i in 0..4 {
            c[i] /= self.factor.powi(2);
        }
        return [c[0] as u8, c[1] as u8, c[2] as u8, c[3] as u8].into()
    }
}