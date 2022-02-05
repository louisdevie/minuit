extern crate minuit;

use minuit::app::App;
use minuit::widget::Widget;

fn main() {
    let demo = App::new(
        "MinUIt demonstration",
        Widget::centered_label("Hello, World!"),
    );

    demo.run();
}
