use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i32_vec(&mut reader);
    let n = line[0];
    let k = line[1];

    let answer = solve(n, k);

    println!("{}", answer);
}

fn solve(a: i32, n: i32) -> i32 {
    let mut tmp = a;
    for _ in 0..n {
        tmp = f(tmp);
    }
    tmp
}

fn f(x: i32) -> i32 {
    g2(x) - g1(x)
}

fn g1(x: i32) -> i32 {
    let mut s: Vec<_> = x.to_string().chars().collect();
    s.sort();
    s.into_iter().collect::<String>().parse::<i32>().unwrap()
}

fn g2(x: i32) -> i32 {
    let mut s: Vec<_> = x.to_string().chars().collect();
    s.sort();
    s.reverse();
    s.into_iter().collect::<String>().parse::<i32>().unwrap()
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
