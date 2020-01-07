use crate::HOST_URL;
use async_std::task;
use crossterm::event::{self, Event, KeyCode};
use std::fs;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};

// TODO

fn is_installed() -> bool {
    fs::read_to_string("/etc/hosts")
        .expect("Error when reading /etc/hosts file")
        .contains("# # Bebasin")
}

fn backup_restore() {
    let home_dir = directories::UserDirs::new();
    let mut hd = home_dir.unwrap().home_dir().to_owned();
    hd.push("backup-hosts");
    fs::remove_file("/etc/hosts").expect("Error when removing /etc/hosts file");
    fs::copy(hd, "/etc/hosts").unwrap();
}

fn download() {
    task::block_on(async {
        let hosts_content = surf::get(HOST_URL)
            .recv_bytes()
            .await
            .expect("Error when retrieving the hosts file");
        fs::File::create("temp-hosts").expect("Error when creating the hosts temprorary file");
        fs::write("temp-hosts", hosts_content)
            .expect("Error when writing to the hosts temprorary file");
    });
}

fn backup() {
    let home_dir = directories::UserDirs::new();
    let mut hd = home_dir.unwrap().home_dir().to_owned();
    hd.push("backup-hosts");
    fs::copy("/etc/hosts", hd).unwrap();
}

pub struct App<'a> {
    menu_items: Vec<&'a str>,
    menu_items_selected: usize,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            menu_items: vec!["Install", "Uninstall"],
            menu_items_selected: 0,
        }
    }

    pub fn dispatch(
        &mut self,
        term: &mut tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
    ) {
        loop {
            term.draw(|mut f| {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(f.size());
                let layout_inner_right = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .margin(10)
                    .split(f.size());

                let block_style = Block::default()
                    .borders(Borders::ALL)
                    .title_style(Style::default().modifier(Modifier::BOLD));
                let style = Style::default().fg(Color::White).bg(Color::Black);
                SelectableList::default()
                    .block(block_style.title("Menu"))
                    .items(&self.menu_items)
                    .select(Some(self.menu_items_selected))
                    .style(style)
                    .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                    .highlight_symbol(">")
                    .render(&mut f, layout[0]);
                let info_text = vec![
                    Text::raw("Bebasin v0.1\n"),
                    Text::raw("Press \"q\" to quit\n"),
                ];
                Paragraph::new(info_text.iter())
                    .block(block_style.title("Information"))
                    .alignment(Alignment::Left)
                    .render(&mut f, layout[1]);
            })
            .unwrap();
            match event::read().unwrap() {
                Event::Key(input) => match input.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Down => {
                        if self.menu_items_selected >= self.menu_items.len() - 1 {
                            self.menu_items_selected = 0;
                        } else {
                            self.menu_items_selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.menu_items_selected > 0 {
                            self.menu_items_selected -= 1;
                        } else {
                            self.menu_items_selected = self.menu_items.len() - 1;
                        }
                    }
                    KeyCode::Enter => {
                        if self.menu_items_selected == 0 {
                            backup();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
