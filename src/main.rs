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
        let but = gtk::Button::new_with_label("Click me");
        but.connect_clicked(|_| {lmao();});
        let bbox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        bbox.add(&but);
        win.add(&bbox);
        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
