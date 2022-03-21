extern crate minuit;

use minuit::prelude::*;

fn main() {
    let mut demo = App::new("MinUIt demonstration");

    let root = demo.set_root(vertical_layout!());

    demo.add_child(root, label!("Hello, World!", CENTER));
    demo.add_child(root, label!("AAAAAAAAAAAAAAAAAAAAAAA"));

    demo.run();
}
