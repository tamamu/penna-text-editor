#[macro_use]
extern crate lazy_static;
extern crate gtk;
extern crate gdk;

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Cell;
use std::sync::Mutex;
use gtk::prelude::*;
use gtk::{Window, WindowType, HeaderBar, Box, Orientation, TextView, ImageMenuItem, MenuButton,
          Image, Popover, TextBuffer, TextIter, AccelGroup, Button};
use gdk::{EventKey, ModifierType};

#[derive(Clone)]
enum ChangeType {
    Insert,
    Delete,
}

#[derive(Clone)]
struct Change {
    action: ChangeType,
    start: i32,
    end: i32,
    text: String,
}

struct Editor {
    undo_pool: Rc<RefCell<Vec<Change>>>,
    redo_pool: Rc<RefCell<Vec<Change>>>,
    view: TextView,
    buffer: TextBuffer, // undo_button: Button,
    user_action: Rc<Cell<bool>>,
}

impl Editor {
    fn new() -> Self {
        let mut undo_pool = Rc::new(RefCell::new(Vec::with_capacity(8192)));
        let mut redo_pool = Rc::new(RefCell::new(Vec::with_capacity(8192)));
        let mut user_action = Rc::new(Cell::new(false));
        let view = gtk::TextView::new();
        let buffer = view.get_buffer().unwrap();
        Editor {
            undo_pool: undo_pool,
            redo_pool: redo_pool,
            view: view,
            buffer: buffer,
            user_action: user_action,
        }
    }

    fn activate(&self) {
        let undo_pool_ptr1 = self.undo_pool.clone();
        let undo_pool_ptr2 = self.undo_pool.clone();
        let redo_pool_ptr1 = self.redo_pool.clone();
        let ua_ptr1 = self.user_action.clone();
        let ua_ptr2 = self.user_action.clone();
        let ua_ptr3 = self.user_action.clone();
        let ua_ptr4 = self.user_action.clone();

        self.buffer.connect_begin_user_action(move |buf: &TextBuffer| {
            ua_ptr1.set(true);
        });

        self.buffer.connect_end_user_action(move |buf: &TextBuffer| {
            ua_ptr2.set(false);
        });

        self.buffer.connect_insert_text(move |buf: &TextBuffer,
                                              iter: &TextIter,
                                              new_text: &str| {
            if ua_ptr3.get() == false {
                return;
            }
            let mut undo_pool = undo_pool_ptr1.borrow_mut();
            let mut redo_pool = redo_pool_ptr1.borrow_mut();
            redo_pool.clear();
            let insert_length = new_text.chars().count() as i32;
            let start_offset = iter.get_offset();
            let end_offset = start_offset + insert_length;
            undo_pool.push(Change {
                action: ChangeType::Insert,
                start: start_offset,
                end: end_offset,
                text: String::from(new_text),
            });
        });

        self.buffer
            .connect_delete_range(move |buf: &TextBuffer, start: &TextIter, end: &TextIter| {
                if ua_ptr4.get() == false {
                    return;
                }
                let mut undo_pool = undo_pool_ptr2.borrow_mut();
                let start_offset = start.get_offset();
                let end_offset = end.get_offset();
                let text = buf.get_slice(start, end, true).unwrap();
                undo_pool.push(Change {
                    action: ChangeType::Delete,
                    start: start_offset,
                    end: end_offset,
                    text: text,
                });
            });


    }

    fn undo(&mut self) {
        let mut undo_pool = self.undo_pool.borrow_mut();
        if undo_pool.len() == 0 {
            return;
        }
        let change = undo_pool.pop().unwrap();
        let mut start_iter = self.buffer.get_iter_at_offset(change.start);
        match change.action {
            ChangeType::Insert => {
                let mut end_iter = self.buffer.get_iter_at_offset(change.end);
                self.buffer.delete(&mut start_iter, &mut end_iter);
            }
            ChangeType::Delete => {
                self.buffer.insert(&mut start_iter, &change.text);
            }
        }
        self.iter_on_screen(&start_iter, "insert");
    }

    fn iter_on_screen(&self, iter: &TextIter, mark_str: &str) {
        self.buffer.place_cursor(iter);
        self.view.scroll_mark_onscreen(&self.buffer.get_mark(mark_str).unwrap());
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
