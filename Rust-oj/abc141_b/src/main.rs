use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let s = read_string(&mut reader);
    let bytes = s.as_bytes();

    let mut result = "Yes";

    for (i, &b) in bytes.iter().enumerate() {
        match i % 2 {
            0 => {
                if b != b'R' && b != b'U' && b != b'D' {
                    result = "No";
                    break;
                }
            }
            1 => {
                if b != b'L' && b != b'U' && b != b'D' {
                    result = "No";
                    break;
                }
            }
            _ => {}
        }
    }

    println!("{}", result);
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
