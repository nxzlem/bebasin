use crate::hosts;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{
    Button, Checkbox, Dialog, DummyView, EditView, Layer, LinearLayout, ListView, SelectView,
    TextView,
};
use cursive::Cursive;

pub struct App {
    cursive: Cursive,
}

fn install(s: &mut Cursive) {
    if hosts::is_installed() {
        s.add_layer(Dialog::info("Bebasin has been installed"));
    } else {
        s.add_layer(Dialog::info("Bebasin is not installed"));
    }
}

fn uninstall(s: &mut Cursive) {
    if hosts::backup_path().exists() {
    } else {
        s.add_layer(Dialog::info("No hosts backup file found!"));
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(format!("Name: {}\nAwesome: yes", name))
            .title(format!("{}'s info", name))
            .button("Quit", Cursive::quit),
    );
}

fn open_repository(s: &mut Cursive) {
    if webbrowser::open("https://github.com/andraantariksa/bebasin").is_err() {
        s.add_layer(
            Dialog::text("Can't open any browser")
                .title("Error")
                .button("Ok", |cur| {
                    cur.pop_layer();
                }),
        );
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            cursive: Cursive::crossterm().unwrap(),
        }
    }

    fn display_main(&mut self) {
        let text_header =
            TextView::new("If you found any issue, please create a new issue on the repository");
        let menu_buttons = LinearLayout::vertical()
            .child(Button::new("Install", install))
            .child(Button::new("Update", uninstall))
            .child(Button::new("Repository", open_repository))
            .child(DummyView)
            .child(Button::new("Quit", Cursive::quit));
        self.cursive.add_layer(
            Dialog::around(
                LinearLayout::vertical()
                    .child(text_header)
                    .child(DummyView)
                    .child(menu_buttons),
            )
            .title("Menu"),
        );
    }

    pub fn dispatch(&mut self) {
        self.cursive.add_global_callback('q', |s| s.quit());
        self.cursive.add_global_callback(Key::Esc, |s| s.quit());
        self.display_main();
        self.cursive.run();
    }
}
