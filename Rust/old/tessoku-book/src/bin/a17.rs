use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_buf: [usize; n-1],
        b_buf: [usize; n-2],
    }

    let mut a = vec![0];
    let mut b = vec![0, 0];

    a.extend(a_buf);
    b.extend(b_buf);

    let mut dp = Vec::new();
    dp.push(0);

    for i in 0..n {
        if i == 0 {
            continue;
        } else if i == 1 {
            dp.push(a[i]);
            continue;
        }

        dp.push(std::cmp::min(a[i] + dp[i - 1], b[i] + dp[i - 2]));
    }

    let mut answer: Vec<i32> = vec![];
    let mut i = n - 1;

    loop {
        answer.insert(0, i as i32 + 1);
        if i == 0 {
            break;
        }

        if dp[i - 1] + a[i] == dp[i] {
            i -= 1;
        } else {
            i -= 2;
        }
    }

    println!("{}", answer.len());
    println!(
        "{}",
        answer
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
