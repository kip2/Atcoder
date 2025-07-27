use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }

    a.sort();

    for x_value in &x {
        println!("{}", a.lower_bound(x_value));
    }
}
