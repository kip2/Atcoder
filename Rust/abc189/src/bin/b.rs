use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
    }

    let mut answer: i64 = -1;
    let mut alcohol = 0;

    for i in 0..n {
        input! {
            v: i64,
            p: i64,
        }
        alcohol += v * p;
        if x * 100 < alcohol {
            answer = i as i64 + 1;
            break;
        }
    }

    println!("{}", answer);
}
