use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a_buf: [usize; n],
    }

    let mut a = vec![0];
    a.extend(a_buf);

    let mut dp = vec![vec![false; s + 1]; n + 1];

    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i] {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                }
            } else if j >= a[i] {
                if dp[i - 1][j] || dp[i - 1][j - a[i]] {
                    dp[i][j] = true;
                }
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
