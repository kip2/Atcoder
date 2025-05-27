use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let s = read_string(&mut reader);

    let (odd_chars, even_chars): (HashSet<_>, HashSet<_>) =
        s.chars().enumerate().partition(|(i, _)| i % 2 == 0);

    let odd_valid = odd_chars
        .into_iter()
        .map(|(_, c)| c)
        .all(|c| matches!(c, 'R' | 'U' | 'D'));

    let even_valid = even_chars
        .into_iter()
        .map(|(_, c)| c)
        .all(|c| matches!(c, 'L' | 'U' | 'D'));

    let result = if odd_valid && even_valid { "Yes" } else { "No" };

    println!("{}", result);
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
