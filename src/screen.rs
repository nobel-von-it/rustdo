use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Text,
    Frame,
};
use serde::{Deserialize, Serialize};

use crate::{
    fs::{read_todos, save_todos},
    todo::Todo,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum State {
    Insert,
    Normal,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    pub todos: Vec<Todo>,
    pub select: usize,
    pub state: State,
}
impl Screen {
    pub fn new(select: Option<usize>) -> Self {
        let select = select.unwrap_or(0);
        Self {
            todos: vec![Todo::new()],
            select,
            state: State::Normal,
        }
    }
    pub fn from_file() -> Self {
        read_todos()
    }
    pub fn save(&mut self) {
        let mut new_todos = vec![];
        for todo in self.todos.iter() {
            if !todo.text.is_empty() {
                new_todos.push(todo.clone());
            }
        }
        self.todos = new_todos;
        save_todos(self);
    }
    pub fn up(&mut self) {
        if self.select > 0 {
            self.select -= 1
        }
    }
    pub fn down(&mut self) {
        if self.select < self.todos.len() - 1 {
            self.select += 1
        }
    }
    pub fn left(&mut self) {
        self.todos[self.select].left();
    }
    pub fn right(&mut self) {
        self.todos[self.select].right();
    }
    pub fn add(&mut self) {
        self.todos.push(Todo::new());
        self.down();
        self.state = State::Insert;
    }
    pub fn remove(&mut self) {
        self.todos.remove(self.select);
        self.up();
    }
    pub fn push(&mut self, c: char) {
        self.todos[self.select].insert(c)
    }
    pub fn pop(&mut self) {
        self.todos[self.select].remove()
    }
    pub fn toggle_done(&mut self) {
        self.todos[self.select].is_done = !self.todos[self.select].is_done;
    }
    pub fn draw(&self, f: &mut Frame) {
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
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
