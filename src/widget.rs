use crate::area::Area;
use crate::property::StringProperty;
use crate::Alignment;

pub type WidgetUId = usize;

#[derive(Debug, Clone)]
pub enum Widget {
    Label {
        area: Area,
        text: StringProperty,
        alignment: super::Alignment,
    },
    FreeLayout {
        area: Area,
        children: Vec<WidgetUId>,
    },
    VerticalLayout {
        area: Area,
        children: Vec<WidgetUId>,
    },
    None,
}

impl Widget {
    pub fn new_label(initial_text: &str, alignment: Alignment, area: Area) -> Self {
        Self::Label {
            area,
            text: StringProperty::initial(initial_text),
            alignment,
        }
    }

    pub fn new_free_layout(area: Area) -> Self {
        Self::FreeLayout {
            area,
            children: vec![],
        }
    }

    pub fn set_text(&mut self, new_text: &str) {
        match self {
            Self::Label { text, .. } => text.set(new_text),
            _ => {}
        }
    }

    pub fn add_child(&mut self, widget: WidgetUId) {
        match self {
            Self::FreeLayout { children, .. } => children.push(widget),
            _ => {}
        }
    }

    pub fn area(&self) -> Area {
        match self {
            Widget::Label { area, .. } => *area,
            Widget::FreeLayout { area, .. } => *area,
            Widget::VerticalLayout { area, .. } => *area,
            Widget::None => Area::new(),
        }
    }
}
