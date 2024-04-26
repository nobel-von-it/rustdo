mod fs;

use crate::fs::{read_todos, save_todos};
use std::{borrow::BorrowMut, io::stdout};

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
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum State {
    Insert,
    Normal,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    text: String,
    is_done: bool,
    position: usize,
}
impl Todo {
    fn new() -> Self {
        Self {
            text: String::new(),
            is_done: false,
            position: 0,
        }
    }
    fn get_pretty(&self) -> String {
        if self.is_done {
            format!(" [*] {}  ", self.text)
        } else {
            format!(" [ ] {}  ", self.text)
        }
    }
    fn left(&mut self) {
        if self.position > 0 {
            self.position -= 1
        }
    }
    fn right(&mut self) {
        if self.position < self.text.len() {
            self.position += 1
        }
    }
    fn insert(&mut self, c: char) {
        if self.text.len() > 100 {
            return;
        }
        self.text.insert(self.position, c);
        self.right();
    }
    fn remove(&mut self) {
        if self.text.len() == 0 {
            return;
        }
        if self.text.len() == self.position {
            self.text.remove(self.position - 1);
        } else {
            self.text.remove(self.position);
        }
        self.left();
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn from_file() -> Self {
        read_todos()
    }
    fn save(&mut self) {
        let mut new_todos = vec![];
        for todo in self.todos.iter() {
            if !todo.text.is_empty() {
                new_todos.push(todo.clone());
            }
        }
        self.todos = new_todos;
        save_todos(self);
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
    fn left(&mut self) {
        self.todos[self.select].left();
    }
    fn right(&mut self) {
        self.todos[self.select].right();
    }
    fn display_selected(&mut self) {}
    fn add(&mut self) {
        self.todos.push(Todo::new());
        self.down();
        self.state = State::Insert;
    }
    fn remove(&mut self) {
        self.todos.remove(self.select);
        self.up();
    }
    fn push(&mut self, c: char) {
        self.todos[self.select].insert(c)
    }
    fn pop(&mut self) {
        self.todos[self.select].remove()
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
    let mut screen = Screen::from_file();

    let res = run(&mut t, &mut screen);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;
    res?;

    println!("{screen:?}");
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
