extern crate async_std;
extern crate crossterm;
extern crate cursive;
#[cfg(not(target_os = "windows"))]
extern crate nix;
extern crate surf;
extern crate webbrowser;
#[cfg(target_os = "windows")]
extern crate winapi;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod app;
mod updater;
mod os;
mod ui;
mod parser;
mod helpers;

const CURRENT_VERSION: u64 = 20200417;
const REPOSITORY_URL: &'static str = "https://github.com/andraantariksa/bebasin";
const LATEST_VERSION_URL: &'static str =
    "https://raw.githubusercontent.com/andraantariksa/bebasin/master/latest.json";
const UPDATE_URL: &'static str =
    "https://api.github.com/repos/andraantariksa/anime4k-rs/releases/latest";
const HOSTS_HEADER: &'static str = include_str!("../misc/header-hosts");
const HOSTS_BEBASIN: &'static str = include_str!("../misc/hosts");

fn main() {
    let mut u = updater::Updater::new();
    u.get_latest_info();
    println!("Updateable {:?}", u.is_updatable());
    println!("{:?}", u.update());

    app::App::new().dispatch();
}
