use crate::renderable::*;
use image::{DynamicImage, GenericImageView, RgbaImage};

#[derive(Debug, Copy, Clone, Default, Hash)]
pub enum UnscalingAlgorithm {
    #[default]
    NearestNeighbour,
    Linear,
    Quadratic,
}

#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone, Debug)]
pub struct LoadImageParams<'a> {
    pub path: Option<&'a str>,
    pub oob_clr: Color,
}

#[allow(non_camel_case_types)]
pub struct load_image {
    image: RgbaImage,
    out_of_bounds_color: Color,
    w: u32,
    h: u32,
}

impl load_image {
    pub fn new(params: LoadImageParams) -> Result<Self, image::ImageError> {
        let image;
        if let Some(path) = params.path {
            image = image::open(path)?.to_rgba8();
        } else {
            image = RgbaImage::new(0, 0)
        }
        let w = image.width();
        let h = image.height();
        Ok(Self {
            image,
            out_of_bounds_color: params.oob_clr,
            w,
            h,
        })
    }
}

impl Renderable for load_image {
    fn render(&self, p: PointInfo) -> Color {
        if p.x.round() >= self.w as f64 || p.x < 0. || p.y.round() >= self.h as f64 || p.y < 0. {
            return self.out_of_bounds_color;
        }
        self.image
            .get_pixel(p.x.round() as u32, p.y.round() as u32)
            .0
            .into()
    }
}
