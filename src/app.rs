extern crate sdl2;

use crate::renderer::Renderer;
use crate::widget::{Widget, WidgetUId};
use sdl2::event::Event;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{Sdl, VideoSubsystem};
use std::time::Duration;

#[derive(Debug)]
pub struct WidgetManager {
    widgets: Vec<Widget>,
    free: Vec<WidgetUId>,
}

#[derive(Debug)]
pub struct WidgetView(Vec<Widget>);

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            free: vec![0],
        }
    }

    pub fn register(&mut self, widget: Widget) -> WidgetUId {
        if self.free.len() == 1 {
            self.widgets.push(widget);
            self.free[0] += 1;
            return self.free[0] - 1;
        } else {
            let insert = self.free.pop().unwrap();
            self.widgets[insert] = widget;
            return insert;
        }
    }

    pub fn get(&self, widget: WidgetUId) -> &Widget {
        self.widgets.get(widget).unwrap()
    }

    pub fn get_mut(&mut self, widget: WidgetUId) -> &mut Widget {
        self.widgets.get_mut(widget).unwrap()
    }

    pub fn get_copy(&self, widget: WidgetUId) -> Widget {
        (*self.widgets.get(widget).unwrap()).clone()
    }

    pub fn handled(&mut self) {
        for widget in &mut self.widgets {
            match widget {
                Widget::Label { text, .. } => text.handled(),
                _ => {}
            }
        }
    }

    pub fn view(&self) -> WidgetView {
        WidgetView(self.widgets.clone())
    }
}

impl WidgetView {
    pub fn get(&self, widget: WidgetUId) -> &Widget {
        self.0.get(widget).unwrap()
    }
}

/// A MinUIt application.
pub struct App {
    /// The sdl context associated with the app
    sdl_context: Sdl,

    /// The sdl video context associated with the app
    sdl_video: VideoSubsystem,

    sdl_font: Sdl2TtfContext,

    /// The name of the app, used as the title of the main window
    name: String,

    /// The widgets of the app
    widgets: WidgetManager,
    root: Option<WidgetUId>,
}

impl App {
    /// Creates a new application instance.
    /// # Arguments
    /// * `name` : the name of the app
    pub fn new(name: &str) -> Self {
        let ctx = sdl2::init().unwrap();
        let vss = ctx.video().unwrap();
        let ttf = sdl2::ttf::init().unwrap();
        Self {
            sdl_context: ctx,
            sdl_video: vss,
            sdl_font: ttf,
            name: String::from(name),
            widgets: WidgetManager::new(),
            root: None,
        }
    }

    /// Runs the main loop of the app.
    /// Should be called at the end of the main function,
    /// once the app is set up.
    pub fn run(&mut self) {
        if self.root == None {
            panic!("There aren't any widgets in the application");
        }

        let window = self
            .sdl_video
            .window(&self.name, 600, 400)
            .position_centered()
            .build()
            .unwrap();

        let mut renderer = Renderer::new(window);
        renderer.font_body = Some(self.sdl_font.load_font("Roboto-Regular.ttf", 12).unwrap());

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let delay = Duration::new(0, 30_000_000u32);

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            renderer.draw(self.root.unwrap(), self.widgets.view());

            self.widgets.handled();

            ::std::thread::sleep(delay);
        }
    }

    pub fn set_root(&mut self, widget: Widget) -> WidgetUId {
        let root_id = self.widgets.register(widget);
        self.root = Some(root_id);
        return root_id;
    }

    pub fn add_child(&mut self, parent: WidgetUId, widget: Widget) -> WidgetUId {
        let child_id = self.widgets.register(widget);
        self.widgets.get_mut(parent).add_child(child_id);
        return child_id;
    }

    pub fn get(&self, widget: WidgetUId) -> &Widget {
        self.widgets.get(widget)
    }
}
