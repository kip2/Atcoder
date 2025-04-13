use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    input! {
        a: [usize; n],
    }

    let mut lr = Vec::new();
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        lr.push((l, r));
    }

    let mut visitors = vec![0; n + 1];
    for i in 0..n {
        visitors[i + 1] = a[i] + visitors[i];
    }

    for i in 0..q {
        let (l, r) = lr[i];
        let answer = visitors[r] - visitors[l - 1];
        println!("{}", answer);
    }
}
