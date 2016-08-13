extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType, HeaderBar, Box, Orientation, Statusbar, TextView, Menu,
          ImageMenuItem, MenuButton, Image, Popover};

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

    let view = TextView::new();
    let buffer = view.get_buffer();

    // let file_menu = Menu::new();
    let open_item = ImageMenuItem::new_with_label("Open");
    let file_menu_button = MenuButton::new();
    // file_menu_button.set_label("File");
    let gears_icon = Image::new_from_icon_name("open-menu", 24);
    let open_icon = Image::new_from_icon_name("document-open", 24);
    let popover = Popover::new(Some(&file_menu_button));
    popover.set_size_request(300, 200);

    file_menu_button.set_image(&gears_icon);
    open_item.set_image(Some(&open_icon));

    // file_menu.attach(&open_item, 0, 1, 0, 1);
    file_menu_button.set_popover(Some(&popover));
    // file_menu.show_all();
    popover.add(&open_item);
    // popover.show_all();


    headerbar.pack_end(&file_menu_button);

    // let statusbar = Statusbar::new();
    window.set_titlebar(Some(&headerbar));
    // hbox.pack_start(&headerbar, false, false, 0);
    hbox.pack_start(&view, true, true, 2);
    // hbox.pack_end(&statusbar, false, false, 0);

    window.add(&hbox);

    window.show_all();
    gtk::main();
}
