use std::path::Path;

pub struct Font {
    glyphs: (),
}

static NORMALIZED_SIZE: f64 = 69.;
/*
impl Font {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let font_data = std::fs::read(path).ok()?;
        let mut face = ttf_parser::Face::parse(&font_data, 0).ok()?;
        if face.is_variable() {
            todo!()
        };
        if face.tables().colr.is_some() {
            if let Some(total) = face.color_palettes() {
                // wtf is this
            }
        };


        let units_per_em = face.units_per_em();
        let scale = NORMALIZED_SIZE / units_per_em as f64;
        let cell_size = face.height() as f64 * NORMALIZED_SIZE / units_per_em as f64;
        if let Some(cmap) = face.tables().cmap {
            for subtable in cmap.subtables.into_iter() {
                for c in 'A'..'Z' {
                    let char_code = c as u32;
                    if let Some(glyph_id) = subtable.glyph_index(char_code) {
                        face.outline_glyph(glyph_id);
                    }
                }
            }

        }
        Some(Self {glyphs: ()})
    }
}
    */

