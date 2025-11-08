use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut x = read_usize_single(&mut reader);
    let n = read_usize_single(&mut reader);

    let w_vec = read_usize_vec(&mut reader);

    let q = read_usize_single(&mut reader);
    let mut p_vec: Vec<usize> = Vec::new();

    for _ in 0..q {
        let p = read_usize_single(&mut reader);
        p_vec.push(p);
    }

    let mut parts_flg: Vec<bool> = vec![false; n];

    for i in 0..q {
        let parts_idx = p_vec[i] - 1;
        let flg = parts_flg[parts_idx];

        if flg {
            x -= w_vec[parts_idx];
            parts_flg[parts_idx] = false;
        } else {
            x += w_vec[parts_idx];
            parts_flg[parts_idx] = true;
        }
        println!("{:?}", x);
    }
}

fn read_usize_single<R: BufRead>(reader: &mut R) -> usize {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<usize>().unwrap()
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
