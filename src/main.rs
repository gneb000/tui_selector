mod tui_selector;

use std::io::stdin;

/// Returns provided vector with respective line numbering at the beginning of each string.
fn add_numbering(entry_list: &Vec<String>) -> Vec<String> {
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

/// Return string with padded number, adjusting string length with zeroes to the left of the
/// provided number so the length matches the biggest number's length (also to be provided).
fn get_num_str(n: usize, max_n: usize) -> String {
    let req_adj = max_n.to_string().len() - n.to_string().len();
    let mut adj_str = String::new();
    for _i in 1..req_adj+1 {
        adj_str.push_str("0");
    }
    adj_str.push_str(n.to_string().as_str());
    adj_str
}

/// Returns formatted content for displaying it in the selector, with line numbering and
/// hiding the ID (if required).
fn prepare_selector_content(input_stream: &Vec<String>, add_num: bool, id_out: bool) -> Vec<String>{
    let mut selector_content = if id_out {
        input_stream.iter()
            .map(|l| l.split_once("::").unwrap().1.to_string())
            .collect()
    } else {
        input_stream.clone()
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

    let selection = tui_selector::select(selector_content);

    if selection.is_some() {
        selection.unwrap()
            .iter()
            .for_each(|i| {
                let mut item: &str = &input_stream[*i];
                if id_out {
                    item = item.split_once("::").unwrap().0
                }
                println!("{}", item);
            });
    }
}
