use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i128_vec(&mut reader);

    let x = line[0].abs();
    let k = line[1];
    let d = line[2];

    let mut answer: i128 = 0;
    if 0 < x - k * d {
        answer = x - k * d;
    } else {
        let rest_count = k - x / d;
        let jump_before = x - d * (x / d);
        let jump_after = jump_before - d;

        if rest_count % 2 == 0 {
            answer = jump_before;
        } else {
            answer = jump_after.abs();
        }
    }

    println!("{}", answer);
}

fn read_i128_vec<R: BufRead>(reader: &mut R) -> Vec<i128> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}
