extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use crate::area::Area;
use crate::property::Property;

pub enum Widget<'w> {
    Label {
        next_sibling: &'w Self,
        area: Area,

        text: Property<String>,
        alignment: super::Alignment,
        texture: Surface<'w>,
    },
    None,
}

impl Widget<'_> {
    pub fn new_label(initial_text: &str, alignment: super::Alignment) -> Self {
        Self::Label {
            next_sibling: &Self::None,
            area: Area::default(),
            text: Property::initial(String::from(initial_text)),
            alignment,
            texture: Surface::new(1, 1, PixelFormatEnum::RGB24).unwrap(),
        }
    }
}
