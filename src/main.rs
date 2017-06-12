extern crate foobar;

use foobar::widgets;

fn main() {
    let mut status = foobar::Status::default();

    status
        .add(widgets::DateTime::default())
        .add(widgets::DateTime::new("%d.%m.%Y"))
        .add(widgets::DateTime::new("%H:%M:%S"));

    status.run().unwrap();
}
