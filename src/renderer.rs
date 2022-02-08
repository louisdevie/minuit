extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

use crate::widget::Widget;

pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        Self {
            canvas: window.into_canvas().build().unwrap(),
        }
    }

    pub fn draw(&mut self, widget: &Widget) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self._draw_widget_recursive(widget);

        self.canvas.present();
    }

    fn _draw_widget_recursive(&mut self, widget: &Widget) {
        match widget {
            Widget::Label { text, .. } => if text.has_changed() {},
            Widget::None => {}
        }
    }
}
