mod selector;

use std::io::stdin;
use selector::selector_ctl;

fn main() {
    let input_stream: Vec<String> = stdin().lines()
        .filter(Result::is_ok)
        .map(|l| l.unwrap().trim().to_string())
        .collect();

    selector_ctl::start_selector(input_stream);
}