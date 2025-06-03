use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_i64_single(&mut reader);

    let result = solve(n);

    print_vec_i64(result);
}

fn print_vec_i64(vector: Vec<i64>) {
    for a in vector {
        println!("{}", a);
    }
}

fn solve(n: i64) -> Vec<i64> {
    let mut remainder = Vec::<i64>::new();

    let max = (n as f64).sqrt().floor() as i64;

    for i in 1..=max {
        if n % i == 0 {
            remainder.push(i);
            let div = n / i;
            if div != i {
                remainder.push((n / i) as i64);
            }
        }
    }

    remainder.sort();

    remainder
}

fn read_i64_single<R: BufRead>(reader: &mut R) -> i64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}
