use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_buf: [usize; n],
    }

    let mut a = vec![0];
    a.extend(a_buf);

    let mut l = 1;
    let mut r = 1000000000;

    while l < r {
        let mid = (l + r) / 2;
        if check(mid, k, n, &a) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    println!("{}", l);
}

fn check(x: usize, k: usize, n: usize, a: &[usize]) -> bool {
    let mut sum = 0;
    for i in 1..=n {
        sum += x / a[i];
    }
    if sum >= k {
        true
    } else {
        false
    }
}
