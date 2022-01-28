extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Duration;

/// A MinUIt application.
pub struct App {
    /// The sdl context associated with the app
	sdl_context: Sdl,

    /// The sdl video context associated with the app
	sdl_video: VideoSubsystem,

    /// The name of the app, used as the title of the main window
	name: String,
}

impl App {
    /// Creates a new application instance.
    /// ## Arguments
    /// * `name` : the name of the app
    pub fn new(name: &str) -> Application {
    	let ctx = sdl2::init().unwrap();
    	let vss = ctx.video().unwrap();
    	Application {
    		sdl_context: ctx,
    		sdl_video: vss,
    		name: String::from(name),
    	}
    }

    /// Runs the main loop of the app.
    /// Should be called at the end of the main function,
    /// once the app is set up.
    pub fn run(&self) {
    	let window = app.sdl_video.window(&app.name, 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = app.sdl_context.event_pump().unwrap();

        let delay = Duration::new(0, 30_000_000u32);

        'running: loop {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {break 'running},
                    _ => {}
                }
            }

            canvas.present();
            ::std::thread::sleep(delay);
        }
    }
}