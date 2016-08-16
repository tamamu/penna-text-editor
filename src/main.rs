#[macro_use]
extern crate lazy_static;
extern crate gtk;
extern crate gdk;

use std::sync::Mutex;
use gtk::prelude::*;
use gtk::{Window, WindowType, HeaderBar, Box, Orientation, TextView, ImageMenuItem, MenuButton,
          Image, Popover, TextBuffer, TextIter, AccelGroup};
use gdk::{EventKey, ModifierType};

struct Change {
    start: i32,
    end: i32,
    text: String,
}

struct AppState {
    history: Vec<Change>,
}

lazy_static! {
    static ref APP: Mutex<AppState> = Mutex::new(AppState{history: Vec::with_capacity(10000)});
}

fn on_insert_text(buf: &TextBuffer, iter: &TextIter, new_text: &str) {
    let insert_length = new_text.chars().count() as i32;
    let start_offset = iter.get_offset();
    let end_offset = start_offset + insert_length;
    APP.lock().unwrap().history.push(Change {
        start: start_offset,
        end: end_offset,
        text: String::from(new_text),
    });
    println!("{}", APP.lock().unwrap().history.len());
}

fn call_undo(buf: &TextBuffer) {
    match APP.lock().unwrap().history.pop() {
        Some(last_change) => {
            let start_iter = buf.get_iter_at_offset(last_change.start);
            let end_iter = buf.get_iter_at_offset(last_change.end);
            buf.select_range(&start_iter, &end_iter);
            buf.delete_selection(false, true);
        }
        None => {}
    }
}

fn on_key_press(view: &TextView, key: &gdk::EventKey) -> Inhibit {
    println!("{}", key.get_keyval());
    let mut result = false;
    match key.get_keyval() {
        117 => {
            if view.im_context_filter_keypress(key) {
                Inhibit(true)
            }
        }
        _ => Inhibit(false),
    }
}

struct MainWindow {
    window: Window,
    headerbar: HeaderBar,
    editor: TextView,
}



fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("*NewText");
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });
    window.set_default_size(640, 480);

    let headerbar = HeaderBar::new();
    headerbar.set_title(Some("*NewText"));
    headerbar.set_subtitle(Some("Untitled new text"));
    headerbar.set_show_close_button(true);

    let hbox = Box::new(Orientation::Vertical, 2);

    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);

    let view = TextView::new();
    view.add_accelerator("activate",
                         accel_group,
                         'u',
                         ModifierType::ControlMask,
                         AccelFlags::Visible);
    let buffer = view.get_buffer().unwrap();
    buffer.connect_insert_text(on_insert_text);

    let open_item = ImageMenuItem::new_with_label("Open");
    let file_menu_button = MenuButton::new();
    let gears_icon = Image::new_from_icon_name("open-menu", 24);
    let open_icon = Image::new_from_icon_name("document-open", 24);
    let popover = Popover::new(Some(&file_menu_button));
    popover.set_size_request(300, 200);

    file_menu_button.set_image(&gears_icon);
    open_item.set_image(Some(&open_icon));

    file_menu_button.set_popover(Some(&popover));
    popover.add(&open_item);
    // popover.show_all();


    headerbar.pack_end(&file_menu_button);

    window.set_titlebar(Some(&headerbar));
    hbox.pack_start(&view, true, true, 2);

    window.add(&hbox);

    window.show_all();
    gtk::main();
}
