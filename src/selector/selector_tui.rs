use std::cmp;
use std::fmt::Display;
use std::io::{Stdout, stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

/// UI and control methods for a text based list item selector.
pub struct SelectorTUI {
    entry_list: Vec<String>,
    stdout: RawTerminal<Stdout>,
    line_idx: usize,
    sel_tracker: Vec<usize>,
    scroll_top: usize,
}

impl SelectorTUI {
    /// Create new instance of SelectorTUI with provided entry list as content.
    pub fn new(entry_list: Vec<String>) -> SelectorTUI {
        SelectorTUI {
            entry_list,
            stdout: stdout().into_raw_mode().unwrap(),
            line_idx: 1,
            sel_tracker: Vec::new(),
            scroll_top: 0
        }
    }

    /// Reloads the content to be displayed, clears the screen & draws the updated content.
    pub fn refresh_content(&mut self) {
        let content = self.make_content();
        let lines_to_draw = self.calculate_lines_to_draw(content);
        self.clear_scr();
        self.draw_content(lines_to_draw);
    }

    /// Moves the cursor down one line. If the bottom is reached, moves cursor to the top.
    pub fn move_down(&mut self) {
        self.line_idx += 1;
        if self.line_idx == self.entry_list.len() + 1 {
            self.go_top();
        }
    }

    /// Moves the cursor up one line. If the top is reached, moves cursor to the bottom.
    pub fn move_up(&mut self) {
        self.line_idx -= 1;
        if self.line_idx < 1 {
            self.go_bottom();
        }
    }

    /// Moves the cursor the the last entry.
    pub fn go_bottom(&mut self) {
        self.line_idx = self.entry_list.len() as usize;
    }

    /// Moves the cursor to the first entry (below the header line).
    pub fn go_top(&mut self) {
        self.line_idx = 1;
    }

    /// Toggle selected status of the entry in current line, by adding respective
    /// line number (entry index in entry_list) to selection_tracker vector.
    pub fn toggle_selection(&mut self) {
        if self.sel_tracker.contains(&(self.line_idx + 1)) {
            let index =
                self.sel_tracker.iter().position(|&x| x == self.line_idx + 1).unwrap();
            self.sel_tracker.remove(index);
        } else {
            self.sel_tracker.push(self.line_idx + 1);
        }
        self.move_down()
    }

    /// Write selected entries to std out.
    pub fn output_selection(&mut self) {
        self.clear_scr();

        // Retrieve and print each of the selected entries
        for (line_num, sel_idx) in self.sel_tracker.iter().enumerate() {
           write!(self.stdout,
                   "{}{}{}{}\n",
                   termion::cursor::Goto(1, (line_num + 1) as u16),
                   termion::color::Fg(termion::color::Reset),
                   termion::color::Bg(termion::color::Reset),
                   self.entry_list[*sel_idx - 2].trim())
                .unwrap();
        }
        self.reset_terminal((self.sel_tracker.len() + 1) as u16);
    }

    /// Clear screen, reset terminal format and set shell prompt position to the top.
    pub fn quit(&mut self) {
        self.clear_scr();
        self.reset_terminal(1);
    }

    /// Clear the screen, adjust cursor position to top-left, hide the cursor.
    fn clear_scr(&mut self) {
        write!(self.stdout,
               "{}{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1),
               termion::cursor::Hide)
            .unwrap();
    }

    /// Restore cursor visibility and position before closing.
    /// Provide line number for the shell prompt to be positioned
    /// after printing output (if any) and closing.
    fn reset_terminal(&mut self, prompt_line: u16) {
        write!(self.stdout,
               "{}{}{}{}",
               termion::cursor::Goto(1, prompt_line),
               termion::color::Fg(termion::color::Reset),
               termion::color::Bg(termion::color::Reset),
               termion::cursor::Show)
            .unwrap();
    }

    /// Iterate through content, drawing each line on screen, flush stdout at the end.
    fn draw_content(&mut self, lines: Vec<String>) {
        for (num, line) in lines.iter().enumerate() {
            self.write_line_stdout((num + 1) as u16, line);
        }
        self.stdout.flush().unwrap();
    }

    /// Calculate amount of lines that fit in the screen, based on terminal height,
    /// and return vector slice with corresponding amount of content lines according
    /// to the scroll level.
    fn calculate_lines_to_draw(&mut self, lines: Vec<String>) -> Vec<String> {
        let term_size = termion::terminal_size().unwrap();
        let max_rows = (term_size.1 - 1) as usize;

        let cur_line = self.line_idx + 1;
        let mut scroll_top = self.scroll_top;
        if cur_line <= scroll_top {
            scroll_top = 0;
        } else if cur_line - scroll_top > max_rows {
            scroll_top = cur_line - max_rows;
        }
        self.scroll_top = scroll_top;

        let last_idx = cmp::min(max_rows, lines.len());
        Vec::from(&lines[scroll_top..scroll_top + last_idx])
    }

    // Writes to std out the provided element to be displayed in the specified line number.
    fn write_line_stdout(&mut self, line_num: u16, display_text: impl Display) {
        write!(self.stdout,
               "{}{}",
               termion::cursor::Goto(1, line_num),
               display_text)
            .unwrap();
    }

    /// Joins header line and entry lines into a vector and returns it.
    fn make_content(&mut self) -> Vec<String> {
        let mut lines = vec!(self.make_header_line());
        lines.append(&mut self.make_entries_into_lines());
        lines
    }

    /// Get String with header line showing 'tagged entry count / total entries' and keybindings.
    fn make_header_line(&mut self) -> String {
        format!(
            "{}{} ({} selected / {} total)  [l/right:select  enter:run selection  q/h/left:quit] ",
            termion::color::Fg(termion::color::Black),
            termion::color::Bg(termion::color::White),
            self.sel_tracker.len(),
            self.entry_list.len()
        )
    }

    /// Build Vec<String> with each line to be displayed from the entry list,
    /// including cursor character '>' positioned in the current line and with
    /// corresponding formatting (one color pair for regular entries and the
    /// reversed color pair for the header and selected entries).
    fn make_entries_into_lines(&mut self) -> Vec<String>{
        let mut lines = Vec::new();
        for (idx, entry) in self.entry_list.iter_mut().enumerate() {
            if self.sel_tracker.contains(&(idx+2)) {
                lines.push(format!(
                    "{}{}{} {}",
                    termion::color::Fg(termion::color::Black),
                    termion::color::Bg(termion::color::White),
                    if (idx+1) == self.line_idx {'>'} else {' '},
                    entry
                ));
            } else {
                lines.push(format!(
                    "{}{}{} {}",
                    termion::color::Fg(termion::color::Reset),
                    termion::color::Bg(termion::color::Reset),
                    if (idx+1) == self.line_idx {'>'} else {' '},
                    entry
                ));
            };
        }
        lines
    }
}
