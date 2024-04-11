use std::{io::stdout, str::FromStr};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Text,
    widgets::Block,
    Frame, Terminal,
};
enum State {
    Insert,
    Normal,
}
#[derive(Debug, Clone)]
struct Todo {
    text: String,
    is_done: bool,
}
impl Todo {
    fn new() -> Self {
        Self {
            text: String::new(),
            is_done: false,
        }
    }
    fn get_pretty(&self) -> String {
        if self.is_done {
            format!(" [*] {}  ", self.text)
        } else {
            format!(" [ ] {}  ", self.text)
        }
    }
}
struct Screen {
    todos: Vec<Todo>,
    select: usize,
    state: State,
}
impl Screen {
    fn new(select: Option<usize>) -> Self {
        let select = select.unwrap_or(0);
        Self {
            todos: vec![Todo::new()],
            select,
            state: State::Normal,
        }
    }
    fn up(&mut self) {
        if self.select > 0 {
            self.select -= 1
        }
    }
    fn down(&mut self) {
        if self.select < self.todos.len() - 1 {
            self.select += 1
        }
    }
    fn add(&mut self) {
        self.todos.push(Todo::new());
    }
    fn remove(&mut self) {
        self.todos.remove(self.select);
    }
    fn push(&mut self, c: char) {
        self.todos[self.select].text.push(c);
    }
    fn pop(&mut self) {
        self.todos[self.select].text.pop();
    }
    fn toggle_done(&mut self) {
        self.todos[self.select].is_done = !self.todos[self.select].is_done;
    }
    fn draw(&self, f: &mut Frame) {
        let mut constraints = vec![];
        for _ in 0..self.todos.len() {
            constraints.push(Constraint::Length(1))
        }
        let full_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Length(1)])
            .split(centered_rect(70, 90, f.size()));

        let todo_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(full_layout[0]);

        let color = Color::Black;
        match self.state {
            State::Insert => f.render_widget(
                Text::raw("Insert").fg(color).on_light_blue(),
                full_layout[1],
            ),
            State::Normal => f.render_widget(
                Text::raw("Normal").fg(color).on_light_green(),
                full_layout[1],
            ),
        };
        for (i, todo) in self.todos.iter().enumerate() {
            if self.select == i {
                let todo_widget = Text::raw(todo.get_pretty()).fg(color).on_white();
                f.render_widget(todo_widget, todo_layout[i]);
            } else {
                let todo_widget = Text::raw(todo.get_pretty()).fg(color).on_gray();
                f.render_widget(todo_widget, todo_layout[i])
            };
        }
    }
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut screen = Screen::new(None);

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
                    KeyCode::Esc => break,
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
