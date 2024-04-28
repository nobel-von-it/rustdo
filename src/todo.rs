use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub text: Vec<char>,
    pub is_done: bool,
    pub pos: usize,
}
impl Todo {
    pub fn new() -> Self {
        Self {
            text: vec!['\0'],
            is_done: false,
            pos: 0,
        }
    }
    pub fn get_text(&self) -> String {
        self.text
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == self.pos {
                    format!("{c}|")
                } else {
                    format!("{c}")
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn get_pretty(&self) -> String {
        if self.is_done {
            format!(" [*] {}  ", self.get_text())
        } else {
            format!(" [ ] {}  ", self.get_text())
        }
    }
    pub fn left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1
        }
    }
    pub fn right(&mut self) {
        if self.pos < self.text.len() - 1 && self.text.len() != 0 {
            self.pos += 1
        }
    }
    pub fn insert(&mut self, c: char) {
        self.text.insert(self.pos + 1, c);
        self.right();
    }
    pub fn remove(&mut self) {
        if self.text.len() > 1 && self.pos > 0 {
            self.text.remove(self.pos);
            self.left();
        }
    }
}
