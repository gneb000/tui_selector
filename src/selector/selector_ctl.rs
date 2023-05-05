use termion::event::Key;
use termion::input::TermRead;

use crate::selector::selector_tui::SelectorTUI;

/// Perform all preparations of the provided entry list, then, call the start
/// of the loop for displaying the selector and listening to user input.
pub fn start_selector(entry_list: Vec<String>) {
    let selector_content = prepare_selector_content(entry_list);
    let tui_selector = make_tui(selector_content);
    start_loop(tui_selector);
}

/// Based on provided entry list, prepare selector content with appropriate
/// line formatting (e.g., add line numbering, ensure consistent spacing).
fn prepare_selector_content(entry_list: Vec<String>) -> Vec<String> {
    let mut selector_content = Vec::new();
    for (idx, entry) in entry_list.iter().enumerate() {
        selector_content.push(format!(
            " {} {}",
            get_num_str(idx+1, entry_list.len()),
            entry.clone(),
        ));
    }
    selector_content
}

/// Instantiate and return TUISelector with provided entry list as displayed content.
fn make_tui(entry_list: Vec<String>) -> SelectorTUI {
    let mut selector_tui = SelectorTUI::new(entry_list);
    selector_tui.refresh_content();
    selector_tui
}

/// Start loop for listening to user input and updating screen accordingly.
fn start_loop(mut tui_selector: SelectorTUI) {
    for c in termion::get_tty().unwrap().keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Left | Key::Char('h') => {
                tui_selector.quit();
                break;
            },
            Key::Up | Key::Char('k') => tui_selector.move_up(),
            Key::Down | Key::Char('j') => tui_selector.move_down(),
            Key::Right | Key::Char('l') => tui_selector.toggle_selection(),
            Key::Char('\n') => {
                tui_selector.output_selection();
                break;
            },
            _ => {}
        }
        tui_selector.refresh_content();
    }
}

/// Return string with padded number, adjusting string length with zeroes to
/// the left of the provided number so the length matches the biggest number's
/// length (also to be provided).
fn get_num_str(n: usize, max_n: usize) -> String {
    let req_adj = max_n.to_string().len() - n.to_string().len();
    let mut adj_str = String::new();
    for _i in 1..req_adj+1 {
        adj_str.push_str("0");
    }
    adj_str.push_str(n.to_string().as_str());
    adj_str
}