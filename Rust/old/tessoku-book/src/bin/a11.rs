use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut a_with_zero = vec![0];
    a_with_zero.extend(a);

    println!("{}", search(x, n, &a_with_zero));
}

fn search(x: usize, n: usize, a: &Vec<usize>) -> i32 {
    let mut l = 0;
    let mut r = n;

    while l <= r {
        let m = (l + r) / 2;
        if x < a[m] {
            r = m - 1
        } else if x > a[m] {
            l = m + 1
        } else if x == a[m] {
            return m as i32;
        }
    }
    -1
}
