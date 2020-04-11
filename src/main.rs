extern crate async_std;
extern crate crossterm;
extern crate cursive;
extern crate directories;
#[cfg(not(target_os = "windows"))]
extern crate nix;
extern crate surf;
extern crate webbrowser;
#[cfg(target_os = "windows")]
extern crate winapi;
extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

mod app;
mod updater;
mod os;
mod ui;
mod parser;

lazy_static! {
    static ref VERSION: u64 = include_str!("../misc/version").parse().unwrap();
}

fn main() {
    app::App::new().dispatch();
}
