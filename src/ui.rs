use crate::hosts;

use cursive::traits::*;
use cursive::views::{
    Button, Checkbox, Dialog, DummyView, EditView, Layer, LinearLayout, ListView, SelectView,
    TextView,
};

use cursive::Cursive;

fn uninstall(s: &mut Cursive) {
    if hosts::backup_path().exists() {
    } else {
        s.add_layer(Dialog::info("No hosts backup file found!"));
    }
}

fn install(cursive: &mut Cursive) {
    cursive.add_layer(Dialog::info(if hosts::is_installed() {
        "Bebasin has been installed"
    } else {
        "Bebasin is not installed"
    }));
}

fn open_repository(cursive: &mut Cursive) {
    if webbrowser::open("https://github.com/andraantariksa/bebasin").is_err() {
        cursive.add_layer(
            Dialog::text("Can't open any browser")
                .title("Error")
                .button("OK", |cursive| {
                    cursive.pop_layer();
                }),
        );
    }
}

pub fn main(cursive: &mut Cursive) {
    let text_header =
        TextView::new("If you found any issue, please create a new issue on the repository");
    let menu_buttons = LinearLayout::vertical()
        .child(Button::new("Install", install))
        .child(Button::new("Update", uninstall))
        .child(Button::new("Repository", open_repository))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));
    cursive.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(text_header)
                .child(DummyView)
                .child(menu_buttons),
        )
        .title("Menu"),
    );
}
