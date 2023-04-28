use std::{io::Stdout, time::Duration};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveUp, RestorePosition, SavePosition},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

struct App {
    pub stdout: Stdout,
    old_y: u16,
    pos_y: u16,
    pos_x: u16,
    tabs: [String; 6],
}

impl App {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
            old_y: 0,
            pos_y: 0,
            pos_x: 0,
            tabs: [
                "e|".to_owned(),
                "B|".to_owned(),
                "G|".to_owned(),
                "D|".to_owned(),
                "A|".to_owned(),
                "E|".to_owned(),
            ],
        }
    }

    fn redraw(&mut self) {
        execute!(self.stdout, MoveUp(self.old_y + 1),).unwrap();

        for line in &self.tabs {
            execute!(
                self.stdout,
                //EnableMouseCapture,
                Print("\r\n"),
                Print(line),
                Clear(ClearType::UntilNewLine),
                Print("\r"),
            )
            .unwrap();
        }

        if self.pos_y == 5 {
            execute!(self.stdout, MoveRight(2 + self.pos_x)).unwrap();
        } else {
            execute!(
                self.stdout,
                MoveUp(5 - self.pos_y),
                MoveRight(2 + self.pos_x)
            )
            .unwrap();
        }

        self.old_y = self.pos_y;
    }
}

fn main() {
    let mut app = App::new();
    enable_raw_mode().unwrap();

    app.redraw();

    loop {
        match crossterm::event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                app.pos_y = app.pos_y.saturating_sub(1);
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if app.pos_y >= 5 {
                    continue;
                }
                app.pos_y += 1;
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                for line in 0..6 {
                    app.tabs[line].insert(app.pos_x as usize + 2, '-');
                }
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                if app.pos_x as usize >= app.tabs[0].len() - 2 {
                    continue;
                }
                app.pos_x += 1;
                for line in 0..6 {
                    app.tabs[line].insert(app.pos_x as usize + 2, '-');
                }
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                app.pos_x = app.pos_x.saturating_sub(1);
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if app.pos_x as usize >= app.tabs[0].len() - 2 {
                    continue;
                }
                app.pos_x += 1;
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                if app.tabs[0].len() == 2 {
                    continue;
                }
                let mut x = app.pos_x;
                if app.pos_x as usize >= app.tabs[0].len() - 2 {
                    x -= 1;
                }
                for line in 0..6 {
                    app.tabs[line].remove(x as usize + 2);
                }
                app.pos_x = x;
                app.redraw();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(mut c),
                ..
            }) => {
                if c == ' ' {
                    c = '-';
                }

                if app.pos_x as usize >= app.tabs[0].len() - 2 {
                    for line in 0..6 {
                        if line == app.pos_y as usize {
                            app.tabs[line].insert(app.pos_x as usize + 2, c);
                        } else {
                            if app.pos_x as usize >= app.tabs[line].len() - 2 {
                                app.tabs[line]
                                    .insert(app.pos_x as usize + 2, if c == '|' { c } else { '-' });
                            }
                        }
                    }
                } else {
                    app.tabs[app.pos_y as usize].replace_range(
                        app.pos_x as usize + 2..app.pos_x as usize + 3,
                        &format!("{c}"),
                    );
                }
                app.pos_x += 1;
                app.redraw();
            }
            //Event::Mouse(event) => println!("{:?}", event),
            //Event::Resize(width, height) => println!("New size {}x{}", width, height),
            _ => {}
        }
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(
        app.stdout,
        MoveDown(5 - app.pos_y),
        Print("\r\n"),
        //DisableMouseCapture
    )
    .unwrap();
}
