use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let values = read_i32_vec(&mut reader);
    let (a, b, c, d) = (values[0], values[1], values[2], values[3]);

    let result = solve(a, b, c, d);
    println!("{}", result);
}

fn solve(a: i32, b: i32, c: i32, d: i32) -> &'static str {
    let c = c - b;
    if c <= 0 {
        return "Yes";
    }

    let a = a - d;
    if a <= 0 {
        return "No";
    }
    solve(a, b, c, d)
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
