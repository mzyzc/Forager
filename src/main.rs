mod feed;

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Grid, MenuBar, MenuItem, Paned, Orientation, ScrolledWindow, TextView, ListBox, ButtonBox, FileChooserButton, FileChooserAction};

fn main() {
    let application = Application::new(
        Some("com.mzyzc.Forage"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|application| {
        init_ui(&application);
    });

    application.run(&[]);
}

fn init_ui(application: &gtk::Application) {
        let window = ApplicationWindow::new(application);
        window.set_title("Forage");
        window.set_default_size(800, 600);

        let grid = Grid::new();
        window.add(&grid);

        let menu_bar = MenuBar::new();
        menu_bar.add(&MenuItem::with_label("File"));
        menu_bar.add(&MenuItem::with_label("View"));
        grid.attach(&menu_bar, 0, 0, 1, 1);

        let paned = Paned::new(Orientation::Horizontal);
        paned.set_hexpand(true);
        paned.set_vexpand(true);
        grid.attach(&paned, 0, 1, 1, 1);

        let scroll_win = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        paned.add1(&scroll_win);

        let text_view = TextView::new();
        paned.add2(&text_view);

        let list_box = ListBox::new();
        scroll_win.add(&list_box);

        let button_box = ButtonBox::new(Orientation::Horizontal);
        grid.attach(&button_box, 0, 3, 1, 1);

        let fc_button = FileChooserButton::new("Select file", FileChooserAction::Open);
        button_box.add(&fc_button);

        window.show_all();

        feed::print_feed();
}