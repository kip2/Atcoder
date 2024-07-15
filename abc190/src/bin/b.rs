use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
    }

    for i in 0..n {
        input! {
            x: usize,
            y: usize,
        }

        if x < s && d < y {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
