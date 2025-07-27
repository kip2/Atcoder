use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i64_vec(&mut reader);
    let a = line[0];
    let b = line[1];

    let answer = lcm(a, b);

    println!("{}", answer);
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == b {
        return b;
    }
    if a > b {
        return gcd(a - b, b);
    } else {
        return gcd(a, b - a);
    }
}

#[test]
fn test_gcd() {
    let expected: i64 = 5;
    let small: i64 = 10;
    let large: i64 = 15;

    let result = gcd(small, large);
    assert_eq!(expected, result);
}

fn read_i64_vec<R: BufRead>(reader: &mut R) -> Vec<i64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
