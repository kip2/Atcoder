use num::integer;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let answer = integer::lcm(a, b);
    println!("{}", answer);
}
