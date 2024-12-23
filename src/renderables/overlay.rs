use crate::renderable::{Color, PointInfo, Renderable};

#[allow(non_camel_case_types)]
pub struct overlay<R1: Renderable, R2: Renderable> {
    pub foreground: R1,
    pub background: R2
}

impl<R1: Renderable, R2: Renderable> Renderable for overlay<R1, R2>{
    fn render(&self, p: PointInfo) -> Color {
        let bc: [u8; 4] = self.background.render(p).into();
        let fc: [u8; 4] = self.foreground.render(p).into();
        let bc_f = [bc[0] as f64 / 255., bc[1] as f64 / 255., bc[2] as f64 / 255., bc[3] as f64 / 255.];
        let fc_f = [fc[0] as f64 / 255., fc[1] as f64 / 255., fc[2] as f64 / 255., fc[3] as f64 / 255.];
        /*let c: [f64; 4] = [
            fc_f[0] * fc_f[3] + bc_f[3]*(1. - fc_f[3]) * bc_f[0],
            fc_f[1] * fc_f[3] + bc_f[3]*(1. - fc_f[3]) * bc_f[1],
            fc_f[2] * fc_f[3] + bc_f[3]*(1. - fc_f[3]) * bc_f[2],
            fc_f[3] + bc_f[3] * (1. - fc_f[3])
        ];*/
        // println!("{:?},{:?}",bc,fc);

        let c: [f64; 4] = [
            fc_f[0] * fc_f[3] + bc_f[3]*(1. - fc_f[3]) * bc_f[0],
            fc_f[1] * fc_f[3] + bc_f[3]*(1. - fc_f[3]) * bc_f[1],
            fc_f[2] * fc_f[3] + bc_f[3]*(1. - fc_f[3]) * bc_f[2],
            fc_f[3] + bc_f[3] * (1. - fc_f[3])
        ];
        // println!("{:?}",c);
        [(c[0] * 255.) as u8, (c[1] * 255.) as u8, (c[2] * 255.) as u8, (c[3] * 255.) as u8].into()
    }
}