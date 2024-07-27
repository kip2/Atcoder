use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_buf: [usize; n],
    }

    let mut a = vec![0];
    a.extend(a_buf);

    let mut r: Vec<usize> = vec![0; a.len()];

    for i in 1..r.len() {
        if i == 1 {
            r[i] = 1
        } else {
            r[i] = r[i - 1]
        };

        while r[i] < n && a[r[i] + 1] - a[i] <= k {
            r[i] += 1;
        }
    }

    let mut answer = 0;

    for i in 1..r.len() {
        answer += r[i] - i;
    }
    println!("{}", answer);
}
