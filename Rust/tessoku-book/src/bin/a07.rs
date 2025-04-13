use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
    }

    let mut lr = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }
        lr.push((l, r));
    }

    let mut arr = vec![0; d + 2];

    for (l, r) in lr {
        arr[l] += 1;
        arr[r + 1] -= 1;
    }

    let mut answer = vec![0; d + 1];

    for i in 1..=d {
        answer[i] = answer[i - 1] + arr[i];
    }

    for i in 1..=d {
        println!("{}", answer[i]);
    }
}
