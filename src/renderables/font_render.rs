use std::panic::RefUnwindSafe;
use crate::renderable::{Color, PointInfo, Renderable};


#[allow(non_camel_case_types)]
pub struct text_opentype {
    // pub font: UnsafeFont,
    pub text: String,
}


impl Renderable for text_opentype {
    fn render(&self, p: PointInfo) -> Color {
        // for i in self.text.chars() {
        //     self.font.font.glyph(i)
        // }
        return [0; 4].into()
    }
}
