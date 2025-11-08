use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_usize_vec(&mut reader);

    let h = line.get(0).unwrap();
    let b = line.get(1).unwrap();

    let mut ans = 0;

    if b < h {
        ans = h - b;
    }

    println!("{:?}", ans);
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
