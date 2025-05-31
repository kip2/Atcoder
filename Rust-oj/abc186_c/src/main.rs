use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_i32_single(&mut reader);

    let result = solve(n);
    println!("{}", result);
}

fn solve(n: i32) -> i32 {
    let mut count = 0;
    let num_str = "7";

    for i in 1..=n {
        if !include_num_from_decimal(i, num_str) && !include_num_from_octal(i, num_str) {
            count += 1;
        }
    }

    count
}

fn include_num_from_decimal(x: i32, num_str: &str) -> bool {
    x.to_string().contains(num_str)
}

fn include_num_from_octal(x: i32, num_str: &str) -> bool {
    let octal_str = format!("{:o}", x);
    octal_str.contains(num_str)
}

fn read_i32_single<R: BufRead>(reader: &mut R) -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}
