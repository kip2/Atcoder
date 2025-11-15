use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_usize_vec(&mut reader);

    let mut max = 0;
    for perm in (0..3).permutations(3) {
        let n = convert_3_digits_number(line[perm[0]], line[perm[1]], line[perm[2]]);
        if n > max {
            max = n;
        }
    }

    println!("{}", max);
}

fn convert_3_digits_number(n1: usize, n2: usize, n3: usize) -> usize {
    let n_100 = n1 * 100;
    let n_10 = n2 * 10;

    n_100 + n_10 + n3
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
