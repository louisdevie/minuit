extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use crate::area::{safe_signed, ResolvedArea};
use crate::widget::Widget;
use crate::Alignment;

fn center(container_size: u32, content_size: u32) -> u32 {
    container_size / 2 - content_size / 2
}

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
            surface_cache: Vec::new(),
        }
    }

    pub fn draw(&mut self, widget: &mut Widget) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        let (client_width, client_height) = self.canvas.output_size().unwrap();
        self._draw_widget_recursive(
            widget,
            ResolvedArea {
                x: 0,
                y: 0,
                w: client_width,
                h: client_height,
            },
        );

        self.canvas.present();
    }

    fn _draw_widget_recursive(&mut self, widget: &mut Widget, parent_area: ResolvedArea) {
        match widget {
            Widget::Label {
                text,
                surface_reference,
                alignment,
                area,
                ..
            } => {
                let resolved = area.resolve(parent_area);

                if text.has_changed() {
                    let surface = self
                        .font_body
                        .as_ref()
                        .unwrap()
                        .render(&text.get())
                        .blended(Color::RGBA(0, 0, 0, 255))
                        .unwrap();
                    if *surface_reference == 0u32 {
                        self.surface_cache.push(surface);
                        *surface_reference = self.surface_cache.len() as u32;
                    } else {
                        self.surface_cache[(*surface_reference - 1) as usize] = surface;
                    }
                    text.handled();
                }

                let texture = self
                    .texture_creator
                    .create_texture_from_surface(
                        &(self.surface_cache[(*surface_reference - 1) as usize]),
                    )
                    .unwrap();
                let TextureQuery { width, height, .. } = texture.query();

                let mut dest_rect =
                    Rect::new(0, safe_signed(center(resolved.h, height)), width, height);
                match alignment {
                    Alignment::LEFT => {}
                    Alignment::RIGHT => {}
                    Alignment::CENTER => dest_rect.set_x(safe_signed(center(resolved.w, width))),
                }

                self.canvas.copy(&texture, None, Some(dest_rect)).unwrap();
            }
            Widget::None => {}
        }
    }
}
