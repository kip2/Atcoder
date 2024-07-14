use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a_buf: [usize; n],
        d: usize,
    }

    let mut a = vec![0];
    a.extend(a_buf);

    let mut left_c_sum: Vec<usize> = vec![0; n + 2];
    let mut right_c_sum: Vec<usize> = vec![0; n + 2];

    for i in 1..=n {
        left_c_sum[i] = max(a[i], left_c_sum[i - 1]);
    }
    for i in (1..=n).rev() {
        right_c_sum[i] = max(a[i], right_c_sum[i + 1]);
    }

    for i in 0..d {
        input! {
            l:usize,
            r:usize,
        }

        println!("{}", max(left_c_sum[l - 1], right_c_sum[r + 1]));
    }
}
