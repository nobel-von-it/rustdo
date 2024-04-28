mod fs;
mod screen;
mod todo;

use crate::screen::Screen;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use screen::State;
use std::io::stdout;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut screen = Screen::from_file();

    let res = run(&mut t, &mut screen);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;
    res?;

    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>, screen: &mut Screen) -> anyhow::Result<()> {
    loop {
        t.draw(|f| screen.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match screen.state {
                State::Normal => match key.code {
                    KeyCode::Esc => {
                        screen.save();
                        break;
                    }
                    KeyCode::Enter | KeyCode::Char('i') => screen.state = State::Insert,
                    KeyCode::Char(' ') | KeyCode::Char('o') => screen.toggle_done(),
                    KeyCode::Char('j') | KeyCode::Down => screen.down(),
                    KeyCode::Char('k') | KeyCode::Up => screen.up(),
                    KeyCode::Char('h') | KeyCode::Left => {
                        // close subtasks
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        // open subtasks
                    }
                    KeyCode::Char('n') => screen.add(),
                    KeyCode::Char('d') => screen.remove(),
                    KeyCode::Char('r') => screen.state = State::Insert,
                    _ => {}
                },
                State::Insert => match key.code {
                    KeyCode::Esc => screen.state = State::Normal,
                    KeyCode::Up => screen.up(),
                    KeyCode::Down => screen.down(),
                    KeyCode::Left => screen.left(),
                    KeyCode::Right => screen.right(),
                    KeyCode::Enter => {
                        // add subtask
                    }
                    KeyCode::Char(c) => screen.push(c),
                    KeyCode::Backspace => screen.pop(),
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
