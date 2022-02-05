extern crate sdl2;

use sdl2::event::Event;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{Sdl, VideoSubsystem};
use std::time::Duration;

use crate::font::FontManager;
use crate::renderer::Renderer;
use crate::widget::Widget;

/// A MinUIt application.
pub struct App<'lt> {
    /// The sdl context associated with the app
    sdl_context: Sdl,

    /// The sdl video context associated with the app
    sdl_video: VideoSubsystem,

    sdl_font: Sdl2TtfContext,

    /// The name of the app, used as the title of the main window
    name: String,

    pub root: Widget<'lt>,

    fonts: FontManager<'lt>,
}

impl App<'_> {
    /// Creates a new application instance.
    /// ## Arguments
    /// * `name` : the name of the app
    pub fn new(name: &str, root: Widget<'static>) -> Self {
        let ctx = sdl2::init().unwrap();
        let vss = ctx.video().unwrap();
        let ttf = sdl2::ttf::init().unwrap();
        let fonts = FontManager::load(&ttf);
        Self {
            sdl_context: ctx,
            sdl_video: vss,
            sdl_font: ttf,
            name: String::from(name),
            root,
            fonts,
        }
    }

    /// Runs the main loop of the app.
    /// Should be called at the end of the main function,
    /// once the app is set up.
    pub fn run(&self) {
        let window = self
            .sdl_video
            .window(&self.name, 600, 400)
            .position_centered()
            .build()
            .unwrap();

        let mut renderer = Renderer::new(window);
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let delay = Duration::new(0, 30_000_000u32);

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            renderer.draw(&self.root);

            ::std::thread::sleep(delay);
        }
    }
}
