extern crate gtk;
extern crate gio;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;
use std::env;
mod create_and_link; 
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
        win.set_property_window_position(gtk::WindowPosition::Center);
        win.set_title("Basic example");
        win.set_border_width(10);
        lmao();
        
        //Create a box
        let notebox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        win.add(&notebox);
        
        let mynote = gtk::Notebook::new();

        let mybox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        win.add(&mybox);

        //Add page to notebook
        let label = gtk::Label::new(Some("Test"));
        mynote.append_page(&mybox, Some(&label));
        
        //Create Stack
        let page_stack = gtk::Stack::new();
        page_stack.set_border_width(6);
        page_stack.set_vexpand(true);
        page_stack.set_hexpand(true);

        //Create Buttons
        let but1 = gtk::Button::new_with_label("Click me");
        but1.connect_clicked(|_| {lmao();});
        
        let but2 = gtk::Button::new_with_label("Click me 2");
        but2.connect_clicked(|_| {lmao();});
        
        page_stack.add_titled(&but1, "P1", "P1");
        page_stack.add_titled(&but2, "P2", "P2");
        
        //Switcher
        let note = gtk::StackSidebar::new();
        note.set_stack(&page_stack);
        
        notebox.pack_start(&mynote, true, true, 0);
        notebox.pack_start(&mybox, true, true, 0);
        mybox.pack_start(&note, true, true, 0);
        mybox.pack_start(&page_stack, true, true, 0);
        // Don't forget to make all widgets visible.
        win.show_all();
        let test_path: Path
        create_base_sym()
    });
    uiapp.run(&env::args().collect::<Vec<_>>());

}
