use std::io::{self, BufRead};

fn main() {
    const MOD: u64 = 1_000_000_007;
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_usize_single(&mut reader);
    let a = read_u64_vec(&mut reader);
    let mut sum_vec: Vec<u64> = vec![];

    // prefix sum
    for i in 0..n {
        match i {
            0 => sum_vec.push(a[i] % MOD),
            _ => {
                let prev = sum_vec[i - 1];
                let current = a[i] % MOD;
                sum_vec.push((prev + current) % MOD);
            }
        }
    }

    let mut total: u64 = 0;
    for i in 0..(n - 1) {
        let right_sum = (MOD + sum_vec[n - 1] % MOD - sum_vec[i] % MOD) % MOD;
        total = (total + (a[i] % MOD) * right_sum) % MOD;
    }

    println!("{}", total);
}

fn read_usize_single<R: BufRead>(reader: &mut R) -> usize {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<usize>().unwrap()
}

fn read_u64_vec<R: BufRead>(reader: &mut R) -> Vec<u64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}
