extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use crate::widget::Widget;

pub struct Renderer<'r> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    pub font_body: Option<Font<'r, 'r>>,
    surface_cache: Vec<Surface<'r>>,
}

impl Renderer<'_> {
    pub fn new(window: Window) -> Self {
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            canvas,
            texture_creator,
            font_body: None,
        }
    }

    pub fn draw(&mut self, widget: &mut Widget) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self._draw_widget_recursive(widget);

        self.canvas.present();
    }

    fn _draw_widget_recursive(&mut self, widget: &mut Widget) {
        match widget {
            Widget::Label {
                text,
                surface_reference,
                ..
            } => {
                if text.has_changed() {
                    let surface = self
                        .font_body
                        .as_ref()
                        .unwrap()
                        .render(&text.get())
                        .blended(Color::RGBA(0, 0, 0, 255))
                        .unwrap();
                    if surface_reference {
                        self.surface_cache[surface_reference] = surface;
                    } else {
                        self.surface_cache.push(surface);
                        *surface_reference = self.surface_cache.len() as u32;
                    }
                }
            }
            Widget::None => {}
        }
    }
}
