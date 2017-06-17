extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate gtk_sys;
#[macro_use] extern crate error_chain;

mod memeit_drawer;
mod gui;

mod errors {
    error_chain! { }
}

use errors::*;


quick_main!(run);

fn run() -> Result<()> {
    gui::run(memeit_drawer::draw)
}
