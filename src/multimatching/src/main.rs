use std;
use std::io;
use std::io::prelude::*;

fn find_matches(line: &String, pattern: &String, i_index: usize, mut occ: String) -> String {
    let index = line[i_index..].find(pattern);
    match index {
        Some(idx) => {
            occ += &((idx + i_index).to_string() + &" ".to_owned());
            if line.len() - idx > pattern.len() {
                return find_matches(&line, &pattern, i_index + idx + 1, occ);
            }
            return occ;
        }
        None => { return occ; }
    }
}

fn main() {
    let input = io::stdin();

    let mut curr_line = 0;
    let mut lines = 0;

    let mut patterns: Vec<String> = vec![];

    for _line in input.lock().lines().map(|_line| _line.ok().unwrap()) {
        
        if curr_line == 0 {
            // Start of test
            lines = _line.parse::<usize>().unwrap();
            curr_line += 1;
        } else if curr_line <= lines {
            patterns.push(_line.to_string());
            curr_line += 1;
        } else {
            // End of test
            for pattern in &patterns {
                let occ = find_matches(&_line, &pattern, 0, String::from(""));
                println!("{}", occ.trim_end());
            }

            // Reset line iterator
            curr_line = 0;
            lines = 0;
            patterns = vec![];
        }   
    }
}
