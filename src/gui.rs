// Separate gui code into own module, this will hopefully make it easy to port to other toolkits if we want this to work on windows

use gtk;
use gtk::*;
use gdk::Display;
use std::rc::Rc;
use std::cell::RefCell;

use ::errors::*;

fn copy_to_clipboard(text: &str) {

    let clipboard = Display::get_default()
        .and_then(|display| Clipboard::get_default(&display))
        .expect("Failed to get clipboard");

    clipboard.set_text(text);
    clipboard.store();
}

type CubeTransformFn = fn(&str, &mut String);


pub fn run(cube_transform_fn: CubeTransformFn) -> Result<()> {
    if gtk::init().is_err() {
        bail!("Failed to initialized gtk");
    }

    let glade_src = include_str!("../res/memeit3d.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).map_err(|_|"Failed to create gui")?;

    let copy_btn: Button = builder.get_object("copy_button").ok_or("Failed to find copy button")?;
    let cube_input: Entry = builder.get_object("cube_input").ok_or("Failed to find input box")?;
    let cube_preview: Label = builder.get_object("cube_preview").ok_or("Failed to find preview")?;

    // Text must be accessed in a mutable state in multiple contexts
    let cube_text = Rc::new(RefCell::new(String::new()));

    cube_input.connect_changed({
        let cube_text = cube_text.clone();
        move |cube_input| {
            let input_text = cube_input.get_text().unwrap();
            let ref mut cube_text = *cube_text.borrow_mut();
            &cube_transform_fn(&input_text, cube_text);
            cube_preview.set_text(&cube_text);
        }
    });

    let copy_and_quit = Rc::new(move || {
        let ref text = *cube_text.borrow();
        copy_to_clipboard(&text);
        gtk::main_quit();
    });

    copy_btn.connect_clicked({
        let copy_and_quit = copy_and_quit.clone();
        move |_| copy_and_quit()
    });

    cube_input.connect_activate(
        move |_| copy_and_quit()
    );

    let app: Window = builder.get_object("app").unwrap();

    app.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    app.show_all();
    gtk::main();
    Ok(())
}
