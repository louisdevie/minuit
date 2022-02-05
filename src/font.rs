use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontManager<'lt> {
    body: Font<'lt, 'lt>,
}

impl FontManager<'_> {
    pub fn load(ttf: &Sdl2TtfContext) -> Self {
        Self {
            body: ttf.load_font("Roboto-Regular.ttf", 12).unwrap(),
        }
    }
}
