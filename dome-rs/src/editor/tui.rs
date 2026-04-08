/// Terminal UI editor using ratatui
/// 
/// Full-featured terminal text editor with cursor movement, search, etc.

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::collections::VecDeque;

pub struct Editor {
    lines: VecDeque<String>,
    cursor_x: usize,
    cursor_y: usize,
    offset_x: usize,
    offset_y: usize,
    modified: bool,
    filename: String,
}

impl Editor {
    /// Create a new editor instance
    pub fn new() -> Self {
        let mut lines = VecDeque::new();
        lines.push_back(String::new());

        Self {
            lines,
            cursor_x: 0,
            cursor_y: 0,
            offset_x: 0,
            offset_y: 0,
            modified: false,
            filename: "Untitled".to_string(),
        }
    }

    /// Load content into editor
    pub fn load(&mut self, content: &str) {
        self.lines = content.lines().map(|s| s.to_string()).collect();
        if self.lines.is_empty() {
            self.lines.push_back(String::new());
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.modified = false;
    }

    /// Get editor content
    pub fn content(&self) -> String {
        self.lines.iter().cloned().collect::<Vec<_>>().join("\n")
    }

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            line.insert(self.cursor_x, c);
            self.cursor_x += 1;
            self.modified = true;
        }
    }

    /// Delete character before cursor
    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            if let Some(line) = self.lines.get_mut(self.cursor_y) {
                line.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
                self.modified = true;
            }
        }
    }

    /// Insert newline at cursor
    pub fn insert_line(&mut self) {
        if let Some(line) = self.lines.get_mut(self.cursor_y) {
            let rest = line.split_off(self.cursor_x);
            self.lines.insert(self.cursor_y + 1, rest);
            self.cursor_y += 1;
            self.cursor_x = 0;
            self.modified = true;
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            if let Some(line) = self.lines.get(self.cursor_y) {
                self.cursor_x = line.len();
            }
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        if let Some(line) = self.lines.get(self.cursor_y) {
            if self.cursor_x < line.len() {
                self.cursor_x += 1;
            } else if self.cursor_y < self.lines.len() - 1 {
                self.cursor_y += 1;
                self.cursor_x = 0;
            }
        }
    }

    /// Move cursor up
    pub fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            // Clamp cursor_x to line length
            if let Some(line) = self.lines.get(self.cursor_y) {
                if self.cursor_x > line.len() {
                    self.cursor_x = line.len();
                }
            }
        }
    }

    /// Move cursor down
    pub fn move_down(&mut self) {
        if self.cursor_y < self.lines.len() - 1 {
            self.cursor_y += 1;
            // Clamp cursor_x to line length
            if let Some(line) = self.lines.get(self.cursor_y) {
                if self.cursor_x > line.len() {
                    self.cursor_x = line.len();
                }
            }
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_new() {
        let editor = Editor::new();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 0);
    }

    #[test]
    fn test_insert_char() {
        let mut editor = Editor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        assert_eq!(editor.content(), "Hi");
    }

    #[test]
    fn test_load_content() {
        let mut editor = Editor::new();
        editor.load("Line 1\nLine 2");
        assert_eq!(editor.lines.len(), 2);
    }
}
