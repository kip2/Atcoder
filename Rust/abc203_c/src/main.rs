use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i64_vec(&mut reader);
    let n = line[0];
    let k = line[1];

    let mut friends = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let line = read_i64_vec(&mut reader);
        let a = line[0];
        let b = line[1];
        friends.push((a, b));
    }
    friends.sort_by_key(|&(a, _)| a);

    let mut pos = k;

    for (a, b) in friends {
        if a <= pos {
            pos += b;
        } else {
            break;
        }
    }

    println!("{}", pos);
}

fn read_i64_vec<R: BufRead>(reader: &mut R) -> Vec<i64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
