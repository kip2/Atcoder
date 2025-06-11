use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i64_vec(&mut reader);
    let a = line[0];
    let b = line[1];

    let mut answer = 0 as i64;
    for i in b..=a * b {
        if i % a == 0 && i % b == 0 {
            answer = i;
            break;
        }
    }
    println!("{}", answer);
}

fn read_i64_vec<R: BufRead>(reader: &mut R) -> Vec<i64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
