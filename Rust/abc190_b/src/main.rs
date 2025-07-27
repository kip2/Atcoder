use std::io::{self, BufRead, StdinLock};

fn main() {
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let line = read_i32_vec(&mut lock);
    let n = line[0];
    let s = line[1];
    let d = line[2];

    let answer = solve(n, s, d, &mut lock);
    println!("{}", answer)
}

fn solve(n: i32, s: i32, d: i32, lock: &mut StdinLock<'static>) -> String {
    let mut answer = "No";

    for _ in 0..n {
        let line = read_i32_vec(lock);
        let x = line[0];
        let y = line[1];

        if x < s && d < y {
            answer = "Yes";
            break;
        }
    }
    answer.to_string()
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
