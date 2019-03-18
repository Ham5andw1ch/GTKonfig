extern crate gtk;
extern crate gio;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;
use std::env;

fn lmao(){
    println!("Lmao");
}


fn main() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"), gio::ApplicationFlags::FLAGS_NONE) .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(320, 200);
        win.set_title("Basic example");
        lmao();
        let but1 = gtk::Button::new_with_label("Click me");
        but1.connect_clicked(|_| {lmao();});
        let but2 = gtk::Button::new_with_label("Click me 2");
        but2.connect_clicked(|_| {lmao();});
        //let bbox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        let page_stack = gtk::Stack::new();
        let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT );
        page_stack.set_border_width(6);
        page_stack.set_vexpand(true);
        page_stack.set_hexpand(true);
        scroll.add(&but1);
        page_stack.add_titled(&scroll, "P1", "P1");
        page_stack.add_titled(&but2, "P2", "P2");
        let note = gtk::StackSwitcher::new();
        note.set_stack(Some(&page_stack));
        //bbox.add(&but);
        //win.add(&bbox);
        win.add(&note);
        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
