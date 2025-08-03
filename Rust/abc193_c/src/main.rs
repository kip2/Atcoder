use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_u64_single(&mut reader);
    let max = (n as f64).sqrt().trunc() as u64;
    let mut set = HashSet::new();

    for a in 2..=max {
        for b in 2..50 {
            if let Some(tmp) = a.checked_pow(b) {
                if tmp <= n {
                    set.insert(tmp);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    println!("{}", n - (set.len() as u64));
}

fn read_u64_single<R: BufRead>(reader: &mut R) -> u64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}
