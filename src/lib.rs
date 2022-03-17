pub mod app;
pub mod area;
pub mod prelude;
pub mod widget;

mod bindata;
mod property;
mod renderer;

#[derive(Debug, Copy, Clone)]
pub enum Alignment {
    LEFT,
    RIGHT,
    CENTER,
}

#[macro_export]
macro_rules! area {
    ($($keyword:ident = $value:expr),* $(,)?) => {
        minuit::area::Area {
            $(
                $keyword: $value,
            )*
            ..minuit::area::Area::new()
        }
    }
}


#[macro_export]
macro_rules! label {
    () => {
        minuit::widget::Widget::new_label(
            "Label",
            minuit::Alignment::RIGHT,
            minuit::area::Area::default(),
        )
    };
    ($text:expr) => {
        minuit::widget::Widget::new_label(
            $text,
            minuit::Alignment::RIGHT,
            minuit::area::Area::default(),
        )
    };
    ($text:expr, $alignment:ident) => {
        minuit::widget::Widget::new_label(
            $text,
            minuit::Alignment::$alignment,
            minuit::area::Area::default(),
        )
    };
    ($text:expr, $area:expr) => {
        minuit::widget::Widget::new_label(
            $text,
            minuit::Alignment::RIGHT,
            $area,
        )
    };
    ($text:expr, $alignment:ident, $area:expr) => {
        minuit::widget::Widget::new_label($text, minuit::Alignment::$alignment, $area)
    };
}

#[macro_export]
macro_rules! free_layout {
    () => {
        minuit::widget::Widget::new_free_layout(minuit::area::Area::default())
    };
}
