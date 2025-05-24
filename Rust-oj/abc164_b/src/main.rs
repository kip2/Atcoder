use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let values = read_i32_vec(&mut reader);
    let (mut a, b, mut c, d) = (values[0], values[1], values[2], values[3]);

    while a > 0 && c > 0 {
        // Turn of TAKAHASHI
        c -= b;
        if c <= 0 {
            println!("Yes");
            return;
        }

        // Turn of AOKI
        a -= d;
        if a <= 0 {
            println!("No");
            return;
        }
    }
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
