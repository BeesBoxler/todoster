use std::path::PathBuf;
use termion::color;

use crate::args::Format;

#[derive(Debug)]
pub struct TodoItem {
    path: PathBuf,
    line_no: usize,
    text: String,
}

impl TodoItem {
    pub fn new(path: PathBuf, line_no: usize, text: String) -> Self {
        TodoItem {
            path,
            line_no,
            text,
        }
    }

    pub fn print(&self, format: &Format) {
        match format {
            Format::Markdown => println!(
                "- [ ] __TODO:__ {} _({}:{})_",
                self.text,
                self.path.to_str().unwrap(),
                self.line_no,
            ),
            Format::Plain => println!(
                "{}:{}:\tTODO: {}",
                self.path.to_str().unwrap(),
                self.line_no,
                self.text
            ),
            Format::Tty => {
                println!(
                    "{}{}{}:{}",
                    color::Fg(color::Yellow),
                    self.path.to_str().unwrap(),
                    color::Fg(color::White),
                    self.line_no,
                );
                println!(
                    "\t{}TODO:{} {}",
                    color::Fg(color::Red),
                    color::Fg(color::White),
                    self.text
                );
                println!();
            }
        }
    }
}
