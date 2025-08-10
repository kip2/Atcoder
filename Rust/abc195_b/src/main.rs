use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

fn main() {
    let kilo = 1000;
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_usize_vec(&mut reader);
    let a = line[0];
    let b = line[1];
    let w = line[2] * kilo;

    let mut min_ans = 10000_00000_00000;
    let mut max_ans: i64 = -10000_00000_00000;

    for i in 1..=1000_000 {
        if a * i <= w && w <= b * i {
            min_ans = min(min_ans, i);
            max_ans = max(max_ans, i as i64);
        }
    }

    if min_ans == 10000_00000_00000 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min_ans, max_ans);
    }
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
