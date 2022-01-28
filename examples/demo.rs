extern crate minuit;

use minuit::app::{app, run};

fn main() {
    let demo = app("MinUIt demonstration");

    run(demo);
}