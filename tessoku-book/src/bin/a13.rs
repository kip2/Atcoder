use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_buf: [usize; n],
    }

    let mut a = vec![0];
    a.extend(a_buf);

    let mut count = 0;
    for i in 1..a.len() {
        let mut l = i + 1;
        let mut r = a.len();
        while l < r {
            let mid = (l + r) / 2;
            if a[i] + k >= a[mid] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        let t = l - 1;
        if t > i && a[i] + k >= a[t] {
            count += t - i;
        }
    }

    println!("{}", count);
}
