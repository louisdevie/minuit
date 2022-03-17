extern crate minuit;

use minuit::prelude::*;

fn main() {
    let mut demo = App::new("MinUIt Markup Language demonstration");

    demo.load_mml_as_root("demo.mml");

    demo.run();
}
