extern crate sdl2;

use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use crate::area::{safe_signed, safe_unsigned, scale, ResolvedArea};
use crate::widget::{Widget, WidgetUId};
use crate::Alignment;

fn center(container_size: u32, content_size: u32) -> u32 {
    container_size / 2 - content_size / 2
}

pub struct Renderer<'r> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    pub font_body: Option<Font<'r, 'r>>,
    surface_cache: HashMap<WidgetUId, Surface<'r>>,
}

impl<'r> Renderer<'r> {
    pub fn new(window: Window) -> Self {
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            canvas,
            texture_creator,
            font_body: None,
            surface_cache: HashMap::new(),
        }
    }

    pub fn draw(&mut self, root: WidgetUId, widgets: Vec<Widget>) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        let (client_width, client_height) = self.canvas.output_size().unwrap();
        self.draw_widget_recursive(
            root,
            ResolvedArea {
                x: 0,
                y: 0,
                w: client_width,
                h: client_height,
            },
            &widgets,
        );

        self.canvas.present();
    }

    fn draw_widget_recursive(
        &mut self,
        widget: WidgetUId,
        parent_area: ResolvedArea,
        widgets: &Vec<Widget>,
    ) {
        match widgets.get(widget).unwrap() {
            Widget::Label {
                text,
                alignment,
                area,
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
                    self.surface_cache.insert(widget, surface);
                }

                let texture = self
                    .texture_creator
                    .create_texture_from_surface(
                        self.surface_cache
                            .get(&widget)
                            .expect("surface disappeared"),
                    )
                    .unwrap();
                let TextureQuery { width, height, .. } = texture.query();

                let mut dest_rect = Rect::new(
                    resolved.x,
                    resolved.y + safe_signed(center(resolved.h, height)),
                    width,
                    height,
                );
                match alignment {
                    Alignment::LEFT => {}
                    Alignment::RIGHT => {}
                    Alignment::CENTER => {
                        dest_rect.set_x(resolved.x + safe_signed(center(resolved.w, width)))
                    }
                }

                self.canvas.copy(&texture, None, Some(dest_rect)).unwrap();
            }
            Widget::FreeLayout { area, children } => {
                let resolved = area.resolve(parent_area);

                for child in children {
                    self.draw_widget_recursive(*child, resolved, widgets);
                }
            }
            Widget::VerticalLayout { area, children } => {
                let resolved = area.resolve(parent_area);
                let absolute: i32 = children.iter().map(|wid| widgets.get(*wid).unwrap().area().absh).sum();
                let remaining = safe_unsigned(safe_signed(resolved.h) - absolute);
                let relative: f32 = children.iter().map(|wid| widgets.get(*wid).unwrap().area().relh).sum();
                println!("{}", relative);
                let mut y = 0;

                for child in children {
                    self.draw_widget_recursive(
                        *child,
                        ResolvedArea {
                            x: resolved.x, 
                            y,
                            w: resolved.w,
                            h: safe_unsigned(
                                scale(
                                    remaining,
                                    widgets.get(*child).unwrap().area().relh / relative,
                                )
                            ),
                        }, 
                        widgets,
                    );
                }
            }
            Widget::None => {}
        }
    }
}
