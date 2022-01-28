extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Duration;

pub struct Application {
	sdl_context: Sdl,
	sdl_video: VideoSubsystem,
	name: String,
}

pub fn app(name: &str) -> Application {
	let ctx = sdl2::init().unwrap();
	let vss = ctx.video().unwrap();
	Application {
		sdl_context: ctx,
		sdl_video: vss,
		name: String::from(name),
	}
}

pub fn run(app: Application) {
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