#[macro_use]
extern crate lazy_static;
extern crate gtk;
extern crate gdk;

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Cell;
use std::sync::Mutex;
use gtk::prelude::*;
use gtk::{Window, WindowType, HeaderBar, Box, Orientation, TextView, ImageMenuItem, MenuButton,
          Image, Popover, TextBuffer, TextIter, AccelGroup, Button};
use gdk::{EventKey, ModifierType};

enum ChangeType {
    Insert,
    Delete,
}

struct Change {
    action: ChangeType,
    start: i32,
    end: i32,
    text: String,
}

struct Editor {
    history: Rc<RefCell<VecDeque<Change>>>,
    history_index: Rc<Cell<usize>>,
    view: TextView,
    buffer: TextBuffer, // undo_button: Button,
}

impl Editor {
    fn new() -> Self {
        let mut history = Rc::new(RefCell::new(VecDeque::with_capacity(10000)));
        let mut history_index = Rc::new(Cell::new(0));
        let view = gtk::TextView::new();
        let buffer = view.get_buffer().unwrap();
        Editor {
            history: history,
            history_index: history_index,
            view: view,
            buffer: buffer,
        }
    }

    fn activate(&self) {
        let history_ptr1 = self.history.clone();
        let hidx_ptr1 = self.history_index.clone();
        self.buffer.connect_insert_text(move |buf: &TextBuffer,
                                              iter: &TextIter,
                                              new_text: &str| {
            let mut history = history_ptr1.borrow_mut();
            let idx = hidx_ptr1.get();
            if idx > 0 {
                *history = history.split_off(idx);
                hidx_ptr1.set(0);
            }
            let insert_length = new_text.chars().count() as i32;
            let start_offset = iter.get_offset();
            let end_offset = start_offset + insert_length;
            history.push_front(Change {
                action: ChangeType::Insert,
                start: start_offset,
                end: end_offset,
                text: String::from(new_text),
            });
        });

        // コメント外すと動かなくなる
        // let history_ptr2 = self.history.clone();
        // let hidx_ptr2 = self.history_index.clone();
        // self.buffer
        // .connect_delete_range(move |buf: &TextBuffer, start: &TextIter, end: &TextIter| {
        // let mut history = history_ptr2.borrow_mut();
        // let idx = hidx_ptr2.get();
        // if idx > 0 {
        // history = history.split_off(idx);
        // hidx_ptr2.set(0);
        // }
        // let start_offset = start.get_offset();
        // let end_offset = end.get_offset();
        // let text = buf.get_slice(start, end, true).unwrap();
        // history.push_front(Change {
        // action: ChangeType::Delete,
        // start: start_offset,
        // end: end_offset,
        // text: text,
        // });
        // });
        //
    }

    fn undo(&mut self) {
        let hidx_ptr = self.history_index.clone();
        match self.history.borrow_mut().get(hidx_ptr.get()) {
            Some(last_change) => {
                match last_change.action {
                    ChangeType::Insert => {
                        let start_iter = self.buffer.get_iter_at_offset(last_change.start);
                        let end_iter = self.buffer.get_iter_at_offset(last_change.end);
                        self.buffer.select_range(&start_iter, &end_iter);
                        self.buffer.delete_selection(false, true);
                        println!("hoge");
                    }
                    ChangeType::Delete => {}
                }
                hidx_ptr.set(hidx_ptr.get() + 1)
            }
            None => {}
        }
    }
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

    let mut editor = Rc::new(RefCell::new(Editor::new()));
    editor.borrow().activate();

    let mut editor_ref = editor.clone();

    let undo_button = Button::new_from_icon_name("edit-undo", 24);
    undo_button.connect_clicked(move |_| {
        editor_ref.borrow_mut().undo();
    });

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


    headerbar.pack_start(&undo_button);
    headerbar.pack_end(&file_menu_button);

    window.set_titlebar(Some(&headerbar));
    hbox.pack_start(&editor.borrow().view, true, true, 2);

    window.add(&hbox);

    window.show_all();
    gtk::main();
}
