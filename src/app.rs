use crate::ui;
use cursive::event::Key;
use cursive::Cursive;
use serde::Deserialize;

#[derive(Deserialize)]
struct Checksum {
    linux: String,
    windows: String,
}

#[derive(Deserialize)]
struct Latest {
    version: u64,
    checksum: Checksum,
}

pub struct App {
    cursive: Cursive,
}

impl App {
    pub fn new() -> Self {
        Self {
            cursive: Cursive::crossterm().unwrap(),
        }
    }

    fn set_global_callback(&mut self) {
        self.cursive.add_global_callback('q', Cursive::quit);
        self.cursive.add_global_callback(Key::Esc, Cursive::quit);
    }

    pub fn dispatch(&mut self) {
        self.set_global_callback();

        ui::main(&mut self.cursive);
        self.cursive.run();
    }
}
