extern crate minuit;

use minuit::prelude::*;

fn main() {
    let mut demo = App::new("MinUIt demonstration");

    let root = demo.set_root(free_layout!());

    demo.add_child(root, label!("Hello, World!", CENTER));
    demo.add_child(
        root,
        label!("Something else.", area!(absh = 20, rely = 1.0, absy = -20)),
    );

    demo.run();
}
