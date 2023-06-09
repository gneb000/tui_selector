use std::cmp;
use std::error::Error;
use std::fmt::Display;
use std::io::{stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// UI and control methods for a text based list item selector.
struct SelectorTUI {
    entry_list: Vec<String>,
    stdout: RawTerminal<Stdout>,
    line_idx: usize,
    sel_tracker: Vec<usize>,
    scroll_top: usize,
}

impl SelectorTUI {
    /// Create new instance of `SelectorTUI` with provided entry list as content.
    pub fn new(entry_list: Vec<String>) -> Result<SelectorTUI, Box<dyn Error>> {
        let selector = SelectorTUI {
            entry_list,
            stdout: stdout().into_raw_mode()?,
            line_idx: 1,
            sel_tracker: Vec::new(),
            scroll_top: 0,
        };
        Ok(selector)
    }

    /// Reloads the content to be displayed, clears the screen and draws the updated content.
    pub fn refresh_content(&mut self) -> Result<(), Box<dyn Error>> {
        let content = self.make_content();
        let lines_to_draw = self.calculate_lines_to_draw(&content);
        self.clear_scr()?;
        self.draw_content(&lines_to_draw)?;
        Ok(())
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
        self.line_idx = self.entry_list.len();
    }

    /// Moves the cursor to the first entry (below the header line).
    pub fn go_top(&mut self) {
        self.line_idx = 1;
    }

    /// Toggle selected status of the entry in current line, by adding respective
    /// line number (entry index in `entry_list`) to `selection_tracker` vector.
    pub fn toggle_selection(&mut self) {
        if self.sel_tracker.contains(&(self.line_idx + 1)) {
            let idx_opt = self.sel_tracker.iter().position(|&x| x == self.line_idx + 1);
            if let Some(index) = idx_opt {
                self.sel_tracker.remove(index);
            }
        } else {
            self.sel_tracker.push(self.line_idx + 1);
        }
        self.move_down();
    }

    /// Select all entries.
    pub fn select_all(&mut self) {
        self.sel_tracker.clear();
        for idx in 0..self.entry_list.len() {
            self.sel_tracker.push(idx + 2);
        }
    }

    /// Deselect all entries.
    pub fn select_none(&mut self) {
        self.sel_tracker.clear();
    }

    /// Returns vector with indices of selected entries.
    pub fn retrieve_selection(&mut self) -> Option<Vec<usize>> {
        if self.sel_tracker.is_empty() {
            return None;
        }
        Some(self.sel_tracker.iter().map(|i| i - 2).collect())
    }

    /// Clear screen, reset terminal format and set shell prompt position to the top.
    pub fn quit(&mut self) -> Result<(), Box<dyn Error>> {
        self.clear_scr()?;
        self.reset_terminal(1)?;
        write!(self.stdout, "{}", termion::cursor::Show)?;
        Ok(())
    }

    /// Clear the screen, adjust cursor position to top-left, hide the cursor.
    fn clear_scr(&mut self) -> Result<(), Box<dyn Error>> {
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )?;
        Ok(())
    }

    /// Restore cursor visibility and position before closing.
    /// Provide line number for the shell prompt to be positioned
    /// after printing output (if any) and closing.
    fn reset_terminal(&mut self, prompt_line: u16) -> Result<(), Box<dyn Error>> {
        write!(
            self.stdout,
            "{}{}{}{}{}",
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset),
            termion::clear::All,
            termion::cursor::Goto(1, prompt_line),
            termion::cursor::Show
        )?;
        Ok(())
    }

    /// Iterate through content drawing each line on screen.
    fn draw_content(&mut self, lines: &[String]) -> Result<(), Box<dyn Error>> {
        for (num, line) in lines.iter().enumerate() {
            self.write_line_stdout(num + 1, line)?;
        }
        self.stdout.flush()?;
        Ok(())
    }

    /// Returns vector with the content lines to draw, determined based on the scroll level
    /// and the amount of lines that fit in the screen depending on terminal height.
    fn calculate_lines_to_draw(&mut self, lines: &[String]) -> Vec<String> {
        let term_size = termion::terminal_size().unwrap_or((120, 40));
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

    // Writes the provided text in the specified line number.
    fn write_line_stdout(&mut self, line_num: usize, display_text: impl Display) -> Result<(), Box<dyn Error>> {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, line_num as u16),
            display_text
        )?;
        Ok(())
    }

    /// Returns vector consolidating header line and entry lines.
    fn make_content(&mut self) -> Vec<String> {
        let mut lines = vec![self.make_header_line()];
        lines.append(&mut self.make_entries_into_lines());
        lines
    }

    /// Returns String with header line showing 'tagged entry count / total entries' and keybindings.
    fn make_header_line(&mut self) -> String {
        format!(
            "{}{} ({} selected / {} total)  [l/right:select  enter:run selection  q/h/left:quit  a:select all  n:deselect all] ",
            termion::color::Fg(termion::color::Black),
            termion::color::Bg(termion::color::White),
            self.sel_tracker.len(),
            self.entry_list.len()
        )
    }

    /// Returns Vec<String> with each line to be displayed from the entry list,
    /// including cursor character '>' positioned in the current line and with
    /// corresponding formatting (one color pair for regular entries and the
    /// reversed color pair for the header and selected entries).
    fn make_entries_into_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        for (idx, entry) in self.entry_list.iter_mut().enumerate() {
            if self.sel_tracker.contains(&(idx + 2)) {
                lines.push(format!(
                    "{}{}{} {}{}{}",
                    termion::color::Fg(termion::color::Black),
                    termion::color::Bg(termion::color::White),
                    if (idx + 1) == self.line_idx { '>' } else { ' ' },
                    entry,
                    termion::color::Fg(termion::color::Reset),
                    termion::color::Bg(termion::color::Reset)
                ));
            } else {
                lines.push(format!(
                    "{}{}{} {}",
                    termion::color::Fg(termion::color::Reset),
                    termion::color::Bg(termion::color::Reset),
                    if (idx + 1) == self.line_idx { '>' } else { ' ' },
                    entry
                ));
            };
        }
        lines
    }
}

/// Returns selected indices, in relation to the provided vector, from the TUI selector.
pub fn select(entry_list: Vec<String>) -> Result<Option<Vec<usize>>, Box<dyn Error>> {
    let mut selection = None;

    let mut tui_selector = SelectorTUI::new(entry_list)?;
    tui_selector.refresh_content()?;
    for c in termion::get_tty()?.keys() {
        match c? {
            Key::Left | Key::Char('q' | 'h') => {
                tui_selector.quit()?;
                break;
            }
            Key::Up | Key::Char('k') => tui_selector.move_up(),
            Key::Down | Key::Char('j') => tui_selector.move_down(),
            Key::Right | Key::Char('l') => tui_selector.toggle_selection(),
            Key::Char('a') => tui_selector.select_all(),
            Key::Char('n') => tui_selector.select_none(),
            Key::Char('\n') => {
                selection = tui_selector.retrieve_selection();
                tui_selector.quit()?;
                break;
            }
            _ => {}
        }
        tui_selector.refresh_content()?;
    }
    Ok(selection)
}
