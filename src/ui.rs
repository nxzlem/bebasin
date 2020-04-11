use crate::updater;
use crate::os::HOSTS_PATH;
use crate::parser::{parse_from_file, write_to_file, parse_from_str};

use cursive::traits::*;
use cursive::views::{
    Button, Checkbox, Dialog, DummyView, EditView, Layer, LinearLayout, ListView, SelectView,
    TextView,
};
use cursive::Cursive;
use std::collections::HashMap;
use std::error::Error;

trait AppendableMap<
    K: std::cmp::Eq + std::hash::Hash,
    V
> {
    fn append(&mut self, other: HashMap<K, Vec<V>>) -> Result<(), ()>;
}

impl<
    K: std::cmp::Eq + std::hash::Hash,
    V
> AppendableMap<K, V> for HashMap<K, Vec<V>> {
    fn append(&mut self, other: HashMap<K, Vec<V>>) -> Result<(), ()> {
        for (key, mut value) in other {
            match self.get_mut(&key) {
                Some(old_val) => {
                    old_val.append(&mut value);
                }
                None => {
                    return Err(());
                }
            }
        }
        Ok(())
    }
}

// fn uninstall(s: &mut Cursive) {
//     if hosts::backup_path().exists() {} else {
//         s.add_layer(Dialog::info("No hosts backup file found!"));
//     }
// }

fn install(cursive: &mut Cursive) {
    let buttons = LinearLayout::vertical()
        .child(Button::new("Merge (Recommended)", install_merge))
        .child(Button::new("Replace", |cursive| {}))
        .child(DummyView)
        .child(Button::new("Cancel", |cursive| { cursive.pop_layer(); }));
    let box_layout = Dialog::around(buttons)
        .title("Select Installation method");

    cursive.add_layer(
        box_layout
    );
}

fn error(cursive: &mut Cursive, err: Box<dyn std::error::Error>) {
    cursive.pop_layer();

    cursive.add_layer(
        Dialog::text(err.to_string())
            .button("Ok", |cursive| {
                cursive.pop_layer();
            })
            .title("Error")
    );
}

fn install_merge(cursive: &mut Cursive) {
    let box_layout = Dialog::text("Parsing the file...")
        .title("Loading...");

    cursive.add_layer(
        box_layout
    );

    match parse_from_str(include_str!("../misc/hosts")) {
        Ok(mut hosts_local) => {
            match parse_from_file("hosts") {
                Ok(hosts_bebasin) => {
                    hosts_local.append(hosts_bebasin);
                    cursive.pop_layer();

                    let box_layout = Dialog::text("Are you sure you want to\n\
                    merge your hosts file with\n\
                    Bebasin hosts?")
                        .title("Confirmation")
                        .button("Confirm", move |cursive| {
                            match write_to_file(HOSTS_PATH, &hosts_local) {
                                Err(err) => {
                                    cursive.add_layer(
                                        Dialog::text(err.to_string())
                                            .title("Error")
                                            .button("Ok", |cursive| {
                                                cursive.pop_layer();
                                                cursive.pop_layer();
                                                cursive.pop_layer();
                                            })
                                    );
                                }
                                _ => {
                                    cursive.add_layer(
                                        Dialog::text("The hosts file has been updated,\n\
                        Please restart your machine")
                                            .title("Done")
                                            .button("Ok", |cursive| {
                                                cursive.pop_layer();
                                                cursive.pop_layer();
                                                cursive.pop_layer();
                                            })
                                    );
                                }
                            };
                        })
                        .button("Cancel", |cursive| { cursive.pop_layer(); });

                    cursive.add_layer(
                        box_layout
                    );
                }
                Err(err) => {
                    error(cursive, Box::new(err));
                }
            };
        }
        Err(err) => {
            error(cursive, Box::new(err));
        }
    };
}

fn open_repository(cursive: &mut Cursive) {
    if webbrowser::open("https://github.com/andraantariksa/bebasin").is_err() {
        let layout = Dialog::text("Can't open any browser")
            .title("Error")
            .button("Ok", |cursive| {
                cursive.pop_layer();
            });

        cursive.add_layer(
            layout
        );
    }
}

pub fn main(cursive: &mut Cursive) {
    let text_header =
        TextView::new("If you found any issue or you have a request,\n\
           please create a new issue on the repository");
    let menu_buttons = LinearLayout::vertical()
        .child(Button::new("Install", install))
        .child(Button::new("Update", |_| {}))
        .child(Button::new("Repository", open_repository))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));
    let layout = Dialog::around(
        LinearLayout::vertical()
            .child(text_header)
            .child(DummyView)
            .child(menu_buttons),
        )
        .title("Menu");

    cursive.add_layer(
        layout
    );
}
