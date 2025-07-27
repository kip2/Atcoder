use std::process::exit;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // If it is divisible by 3, there is no need to remove the digit.
    let rem_all = n % 3;
    if rem_all == 0 {
        println!("0");
        exit(0);
    }

    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut rem_1 = false;
    let mut rem_2 = false;

    for d in digits {
        if d % 3 == 1 {
            rem_1 = true;
        } else if d % 3 == 2 {
            rem_2 = true;
        }
    }

    if rem_all == 1 {
        if rem_1 {
            if n.to_string().len() <= 1 {
                println!("-1");
            } else {
                println!("1");
            }
        } else {
            if n.to_string().len() <= 2 {
                println!("-1");
            } else {
                println!("2");
            }
        }
    } else if rem_all == 2 {
        if rem_2 {
            if n.to_string().len() <= 1 {
                println!("-1");
            } else {
                println!("1");
            }
        } else {
            if n.to_string().len() <= 2 {
                println!("-1");
            } else {
                println!("2");
            }
        }
    }
}
