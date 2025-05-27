use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let s = read_string(&mut reader);

    let odd_chars: HashSet<char> = s
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c)
        .collect::<Vec<char>>()
        .into_iter()
        .collect();

    let even_chars: HashSet<char> = s
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, c)| c)
        .collect::<Vec<char>>()
        .into_iter()
        .collect();

    let mut result = "Yes";

    for c in odd_chars {
        if c != 'R' && c != 'U' && c != 'D' {
            result = "No";
            break;
        }
    }

    for c in even_chars {
        if c != 'L' && c != 'U' && c != 'D' {
            result = "No";
            break;
        }
    }

    println!("{}", result);
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
