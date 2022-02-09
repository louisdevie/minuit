use crate::area::Area;
use crate::property::StringProperty;

pub enum Widget<'w> {
    Label {
        next_sibling: &'w Self,
        area: Area,

        text: StringProperty,
        alignment: super::Alignment,
        surface_reference: u32,
    },
    None,
}

impl Widget<'_> {
    pub fn new_label(initial_text: &str, alignment: super::Alignment) -> Self {
        Self::Label {
            next_sibling: &Self::None,
            area: Area::default(),
            text: StringProperty::initial(initial_text),
            alignment,
            surface_reference: 0,
        }
    }

    pub fn set_text(&mut self, new_text: &str) {
        match self {
            Self::Label { text, .. } => text.set(new_text),
            Self::None => {}
        }
    }
}
