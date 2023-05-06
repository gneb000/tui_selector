mod tui_selector;

use std::io::stdin;

/// Returns provided vector with respective line numbering at the beginning of each string.
fn add_numbering(entry_list: &[String]) -> Vec<String> {
    entry_list
        .iter()
        .enumerate()
        .map(|(i, e)| format!(" {} {}", get_num_str(i+1, entry_list.len()), e.clone()))
        .collect()
}

/// Returns string with padded number, adjusting string length with zeroes to the left of the
/// provided number so the length matches the biggest number's length (also to be provided).
fn get_num_str(n: usize, max_n: usize) -> String {
    let req_adj = max_n.to_string().len() - n.to_string().len();
    let mut adj_str: String = (1..=req_adj).map(|_| '0').collect();
    adj_str.push_str(&n.to_string());
    adj_str
}

/// Returns formatted content for displaying it in the selector, with line numbering and
/// hiding the ID (if required).
fn prepare_selector_content(input_stream: &[String], add_num: bool, id_out: bool) -> Vec<String>{
    let mut selector_content = if id_out {
        input_stream.iter()
            .map(|l| l.split_once("::").unwrap().1.to_string())
            .collect()
    } else {
        input_stream.to_owned()
    };

    if add_num {
        selector_content = add_numbering(&selector_content);
    }

    selector_content
}

fn main() {
    let input_stream: Vec<String> = stdin().lines()
        .filter(Result::is_ok)
        .map(|l| l.unwrap().trim().to_string())
        .collect();

    // TODO - Replace with CLAP args
    let numbering = true;
    let id_out = false;
    // // // // // // // // // // //

    let selector_content = prepare_selector_content(&input_stream, numbering, id_out);

    let selected_indices = tui_selector::select(selector_content);

    if let Some(selection) = selected_indices {
        for i in selection {
            let mut item: &str = &input_stream[i];
            if id_out {
                item = item.split_once("::").unwrap().0;
            }
            println!("{item}");
        }
    }
}
