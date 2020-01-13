use crate::ui;

use cursive::event::Key;
use cursive::Cursive;

pub struct App {
    cursive: Cursive,
}

impl App {
    pub fn new() -> Self {
        Self {
            cursive: Cursive::crossterm().unwrap(),
        }
    }

    pub fn dispatch(&mut self) {
        self.cursive.add_global_callback('q', |s| s.quit());
        self.cursive.add_global_callback(Key::Esc, |s| s.quit());
        ui::main(&mut self.cursive);
        self.cursive.run();
    }
}
