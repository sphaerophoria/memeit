extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate gtk_sys;
#[macro_use] extern crate error_chain;

mod memeit_drawer;

mod errors {
    error_chain! { }
}

use gtk::*;
use gdk::Display;
use std::rc::Rc;
use std::cell::RefCell;
use memeit_drawer::MemeitDrawer;
use errors::*;


fn copy_to_clipboard(text: &str) {

    let clipboard = Display::get_default()
        .and_then(|display| Clipboard::get_default(&display))
        .expect("Failed to get clipboard");

    clipboard.set_text(text);
    clipboard.store();
}

quick_main!(run);

fn run() -> Result<()> {
    if gtk::init().is_err() {
        bail!("Failed to initialized gtk");
    }

    let glade_src = include_str!("../res/memeit3d.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();//.chain_err("Failed to create gui");

    let app: Window = builder.get_object("app").unwrap();
    let copy_btn: Button = builder.get_object("copy_button").unwrap();
    let meme_entry: Rc<Entry> = Rc::new(builder.get_object("meme_entry").unwrap());
    let meme_display_src: Rc<TextView> = Rc::new(builder.get_object("meme_display").unwrap());
    let current_meme_text_src: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

    let current_meme_text = current_meme_text_src.clone();
    let meme_display = meme_display_src.clone();
    meme_entry.connect_changed(move |meme_entry| {
        let text = meme_entry.get_text();
        let buffer = meme_display.get_buffer().unwrap();
        if text.is_none() {
            buffer.set_text("");
            return;
        }
        let text = text.unwrap();

        if text.len() < 4 {
            buffer.set_text("");
            return;
        }

        let memeit_drawer = MemeitDrawer::new(&text.to_uppercase());
        let ref mut current_meme_text = *current_meme_text.borrow_mut();
        current_meme_text.clear();
        current_meme_text.push_str(&memeit_drawer.to_string());
        meme_display.get_buffer().unwrap().set_text(&current_meme_text);
    });

    let current_meme_text = current_meme_text_src.clone();
    copy_btn.connect_clicked(move |_| {
        let ref text = *current_meme_text.borrow();
        copy_to_clipboard(&text);
        gtk::main_quit();
    });

    let current_meme_text = current_meme_text_src.clone();
    meme_entry.connect_activate(move |_| {
        let ref text = *current_meme_text.borrow();
        copy_to_clipboard(&text);
        gtk::main_quit();
    });

    app.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    app.show_all();

    gtk::main();

    Ok(())

}
