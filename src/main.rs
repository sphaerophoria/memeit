extern crate gtk;
extern crate gdk;
#[macro_use] extern crate error_chain;

use gtk::*;
use std::rc::Rc;
use std::cell::RefCell;

mod errors {
    error_chain! { }
}

use errors::*;

fn idx2(x: usize, y: usize, pitch: usize) -> usize {
    y * pitch + x
}

struct MemeitDrawer {
    pitch: usize,
    len: usize,
    input: String,
    canvas: Vec<char>,
}

impl MemeitDrawer {
    pub fn new(input: &str) -> MemeitDrawer {
        let len = input.chars().count();
        let mut canvas = Vec::new();

        let width = len * 3;
        let height = len * 3 / 2;
        for _ in 0..height {
            for _ in 0..width {
                canvas.push(' ');
            }
            canvas.push('\n');
        }
        canvas.pop();

        MemeitDrawer {
            pitch: width + 1,
            len: len,
            input: input.to_string(),
            canvas: canvas,
        }
    }

    pub fn draw(&mut self) {
        let len = self.len;
        self.draw2d(if len % 2  == 0{len } else { len - 1}, 0);
        self.draw2d(0, len / 2);
        self.draw_diagonals();
    }

    fn draw2d(&mut self, start_x: usize, start_y: usize) {
        let len = self.len;
        for (i, c) in self.input.chars().enumerate() {
            // Top row
            self.canvas[idx2(i * 2 + start_x, start_y, self.pitch)] = c;
            // Left column
            self.canvas[idx2(start_x, i + start_y, self.pitch)] = c;
            // Bottom row
            self.canvas[idx2((len - 1) * 2 - (i * 2) + start_x, start_y + len - 1, self.pitch)] = c;
            // Right column
            self.canvas[idx2((len - 1) * 2 + start_x ,start_y + len - 1 - i, self.pitch)] = c;
        }
    }

    fn draw_diagonals(&mut self) {
        let len = self.len;
        for i in 1..len/2 {
            self.canvas[idx2((len / 2 - i) * 2, i, self.pitch)] = '/';
            self.canvas[idx2((len / 2 - i) * 2, i + len - 1, self.pitch)] = '/';
            self.canvas[idx2((len / 2 - i) * 2 + len * 2 - 2, i, self.pitch)] = '/';
            self.canvas[idx2((len / 2 - i) * 2 + len * 2 - 2, i + len - 1, self.pitch)] = '/';
        }
    }
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

        let mut memeit_drawer = MemeitDrawer::new(&text.to_uppercase());
        memeit_drawer.draw();
        let ref mut current_meme_text = *current_meme_text.borrow_mut();
        current_meme_text.clear();
        current_meme_text.push_str(&memeit_drawer.canvas.into_iter().collect::<String>());
        meme_display.get_buffer().unwrap().set_text(&current_meme_text);
    });

    let current_meme_text = current_meme_text_src.clone();
    copy_btn.connect_clicked(move |_| {
        let ref text = *current_meme_text.borrow();
        let clipboard = Clipboard::get_default(&gdk::Display::get_default().unwrap()).unwrap();
        clipboard.set_text(&text);
        clipboard.store();
        gtk::main_quit();
    });

    let current_meme_text = current_meme_text_src.clone();
    meme_entry.connect_activate(move |_| {
        let ref text = *current_meme_text.borrow();
        let clipboard = Clipboard::get_default(&gdk::Display::get_default().unwrap()).unwrap();
        clipboard.set_text(&text);
        clipboard.store();
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
