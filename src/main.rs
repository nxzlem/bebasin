extern crate crossterm;
extern crate cursive;
#[cfg(any(target_os = "linux", target_os = "macos"))]
extern crate nix;
extern crate pest;
extern crate webbrowser;
#[cfg(target_os = "windows")]
extern crate winapi;
#[macro_use]
extern crate pest_derive;
extern crate zip;

mod app;
mod helpers;
mod os;
mod parser;
mod ui;
mod updater;

const CURRENT_VERSION: u64 = 20200601;
const REPOSITORY_URL: &'static str = "https://github.com/bebasid/bebasin";
const LATEST_VERSION_URL: &'static str =
    "https://raw.githubusercontent.com/bebasid/bebasin/master/latest.json";
const UPDATE_URL: &'static str =
    "https://api.github.com/repos/bebasid/bebasin/releases/latest";
const HOSTS_HEADER: &'static str = include_str!("../misc/header-hosts");
const HOSTS_BEBASIN: &'static str = include_str!("../misc/hosts");

fn main() {
    // let mut u = updater::Updater::new();
    // u.get_latest_info();
    // println!("Updateable {:?}", u.is_updatable());
    // println!("{:?}", u.update());
    updater::remove_temp_file();

    app::App::new().dispatch();
}
