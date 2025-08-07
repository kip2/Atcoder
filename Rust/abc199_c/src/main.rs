use std::io::{self, BufRead};

#[derive(PartialEq)]
enum ReverseFlag {
    Reverse,
    NotReverse,
}

impl ReverseFlag {
    fn toggle(&mut self) {
        *self = match self {
            ReverseFlag::NotReverse => ReverseFlag::Reverse,
            ReverseFlag::Reverse => ReverseFlag::NotReverse,
        }
    }

    fn convert_index(&self, i: usize, n: usize) -> usize {
        match self {
            ReverseFlag::NotReverse => i,
            ReverseFlag::Reverse => {
                if i < n {
                    i + n
                } else {
                    i - n
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_u64_single(&mut reader);
    let mut s: Vec<char> = read_string(&mut reader).chars().collect();
    let q = read_u64_single(&mut reader);
    let mut flg = ReverseFlag::NotReverse;

    for _ in 0..q {
        let line = read_usize_vec(&mut reader);
        let t = line[0];
        let mut a = line[1] - 1;
        let mut b = line[2] - 1;

        match t {
            1 => {
                a = flg.convert_index(a, n as usize);
                b = flg.convert_index(b, n as usize);
                s.swap(a, b);
            }
            2 => flg.toggle(),
            _ => (),
        }
    }

    if flg == ReverseFlag::Reverse {
        rotate_half(&mut s);
    }

    println!("{}", s.iter().collect::<String>());
}

fn rotate_half(s: &mut Vec<char>) {
    let half = s.len() / 2;
    s.rotate_left(half);
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_u64_single<R: BufRead>(reader: &mut R) -> u64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
