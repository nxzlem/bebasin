extern crate async_std;
extern crate crossterm;
extern crate cursive;
extern crate directories;
extern crate nix;
extern crate structopt;
extern crate surf;
extern crate webbrowser;
#[cfg(target_os = "windows")]
extern crate winapi;

mod app;
mod hosts;
mod os;

use structopt::StructOpt;

// Should be set as a program release file for update
#[allow(dead_code)]
const UPDATE_URL: &'static str =
    "https://raw.githubusercontent.com/gvoze32/bebasid/master/releases/hosts";

#[derive(StructOpt, Debug)]
#[structopt()]
struct ArgsOpt {}

fn main() {
    let _ = ArgsOpt::from_args();

    // Program should be run using adminstrator access in order to modify hosts file
    // if os::is_has_admin_access() {
    //     print!("Requires root access. Try to run it with sudo");
    //     std::process::exit(0);
    // }
    app::App::new().dispatch();
}
