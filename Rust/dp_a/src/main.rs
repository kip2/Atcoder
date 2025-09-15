use std::{
    cmp::min,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // sample: get input
    let _n = read_isize_single(&mut reader);
    let h = read_isize_vec(&mut reader);

    let mut dp: Vec<isize> = vec![0; h.len()];

    for i in 0..h.len() {
        dp[i] = match i {
            0 => 0,
            1 => diff_h(h[i - 1], h[i]),
            _ => {
                let p1 = dp[i - 1] + diff_h(h[i - 1], h[i]);
                let p2 = dp[i - 2] + diff_h(h[i - 2], h[i]);
                min(p1, p2)
            }
        }
    }

    println!("{}", dp.last().expect("dp is empty."));
}

fn diff_h(h1: isize, h2: isize) -> isize {
    (h2 - h1).abs()
}
fn read_isize_single<R: BufRead>(reader: &mut R) -> isize {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<isize>().unwrap()
}

fn read_isize_vec<R: BufRead>(reader: &mut R) -> Vec<isize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}
