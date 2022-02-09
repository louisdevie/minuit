extern crate minuit;

use minuit::app::App;
use minuit::widget::Widget;
use minuit::Alignment;

fn main() {
    let mut demo = App::new(
        "MinUIt demonstration",
        Widget::new_label("Hello, World!", Alignment::LEFT),
    );

    demo.run();
}
