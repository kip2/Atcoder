use std::io::{self, BufRead};

fn main() {
    let s = read_string();

    let mut chars = s.chars();
    let head = chars.next().unwrap();
    let rest = chars.collect::<String>();

    let answer = format!("{}{}", rest, head);
    println!("{}", answer)
}

fn read_string() -> String {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    read_string_from(&mut lock)
}

fn read_string_from<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
