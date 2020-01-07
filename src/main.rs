extern crate async_std;
extern crate crossterm;
extern crate directories;
extern crate nix;
extern crate structopt;
extern crate surf;
extern crate tui;

const HOST_URL: &str = "https://raw.githubusercontent.com/gvoze32/bebasid/master/releases/hosts";

mod app;

use crossterm::{execute, style, terminal};
use std::io::{self, Write};
use structopt::StructOpt;
use tui::{backend::CrosstermBackend, Terminal};

#[derive(StructOpt, Debug)]
#[structopt()]
struct ArgsOpt {}

fn main() {
    let _opt = ArgsOpt::from_args();

    if !nix::unistd::geteuid().is_root() {
        print!("Requires root access. Try to run it with sudo");
        std::process::exit(0);
    }

    let mut stdout = io::stdout();
    execute!(
        &mut stdout,
        terminal::EnterAlternateScreen,
        style::ResetColor,
    )
    .unwrap();
    let term_backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(term_backend).unwrap();
    term.clear().unwrap();
    terminal::enable_raw_mode().unwrap();
    term.hide_cursor().unwrap();

    let mut app = app::App::new();
    app.dispatch(&mut term);

    execute!(term.backend_mut(), terminal::LeaveAlternateScreen).unwrap();
    term.show_cursor().unwrap();
    terminal::disable_raw_mode().unwrap();
}
