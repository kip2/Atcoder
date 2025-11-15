use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // let x = read_usize_single(&mut reader);
    let x = read_string(&mut reader);
    let x_size = x.len();
    let x_vec_char: Vec<char> = x.chars().collect();
    let x_digits = 10_u32.pow(x_size as u32 - 1);

    let mut min: usize = 100_000_000_00;
    for perm in (0..x_size).permutations(x_size) {
        let n = convert_n(&x_vec_char, &perm);

        if n < x_digits as usize {
            continue;
        }

        if n < min {
            min = n;
        }
    }

    println!("{:?}", min);
}

fn convert_n(chars: &Vec<char>, perm: &Vec<usize>) -> usize {
    let mut s = Vec::<char>::new();
    for i in perm {
        s.push(chars[*i]);
    }
    let b: String = s.iter().collect();
    b.parse().unwrap()
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
