use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_usize_vec(&mut reader);
    let n = line[0];
    let m = line[1];
    let k = line[2];

    let mut h_vec = read_usize_vec(&mut reader);
    let mut b_vec = read_usize_vec(&mut reader);

    h_vec.sort();
    b_vec.sort();

    let mut cnt = 0;

    let mut answer = "No";

    for b_i in 0..b_vec.len() {
        for h_i in 0..h_vec.len() {
            let creatable = h_vec[h_i] <= b_vec[b_i];

            if creatable {
                h_vec.remove(h_i);
                cnt += 1;
                break;
            }
        }

        if cnt >= k {
            answer = "Yes";
            break;
        };
    }
    println!("{}", answer);
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
