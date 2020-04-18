use crate::{updater, REPOSITORY_URL, HOSTS_HEADER, HOSTS_BEBASIN};
use crate::os::{HOSTS_PATH, HOSTS_BACKUP_PATH};
use crate::parser::{parse_from_file, write_to_file, parse_from_str, ErrorKind};

use cursive::traits::*;
use cursive::views::{
    Button, Checkbox, Dialog, DummyView, EditView, Layer, LinearLayout, ListView, SelectView,
    TextView,
};
use cursive::Cursive;

use crate::helpers::AppendableMap;
use crate::updater::{is_backed, backup};

fn error(cursive: &mut Cursive, err: ErrorKind) {
    cursive.pop_layer();

    cursive.add_layer(
        Dialog::text(err.to_string())
            .button("Ok", |cursive| {
                cursive.pop_layer();
            })
            .title("Error")
    );
}

fn install(cursive: &mut Cursive) {
    let box_layout = Dialog::text("Parsing the file...")
        .title("Loading...");

    cursive.add_layer(
        box_layout
    );

    if !is_backed() {
        let backup_result = backup();
        if backup_result.is_err() {
            error(cursive, backup_result.err().unwrap());
            return;
        }
    }

    match parse_from_str(HOSTS_BEBASIN) {
        Ok(mut hosts_bebasin) => {
            match parse_from_file(HOSTS_BACKUP_PATH) {
                Ok(hosts_backup) => {
                    hosts_bebasin.append(hosts_backup);
                    cursive.pop_layer();

                    let box_layout = Dialog::text("Are you sure you want to\n\
                    merge your hosts file with\n\
                    Bebasin hosts?")
                        .title("Confirmation")
                        .button("Confirm", move |cursive| {
                            match write_to_file(HOSTS_PATH, &hosts_bebasin, HOSTS_HEADER) {
                                Err(err) => {
                                    cursive.add_layer(
                                        Dialog::text(err.to_string())
                                            .title("Error")
                                            .button("Ok", |cursive| {
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
                    error(cursive, err);
                }
            };
        }
        Err(err) => {
            error(cursive, err);
        }
    };
}

fn open_repository(cursive: &mut Cursive) {
    if webbrowser::open(REPOSITORY_URL).is_err() {
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
