use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut deposit = 100;
    let mut years = 0;
    let x = read_i64_single(&mut reader);

    while deposit < x {
        deposit += deposit / 100;
        years += 1;
    }

    println!("{}", years);
}

fn read_i64_single<R: BufRead>(reader: &mut R) -> i64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}
