use proconio::input;

fn main() {
    // inputの取得

    // aに関しては0だけのvecを作成し、受け取ったbufを末尾に追加する

    // dp用の配列を用意する

    // dp[0][0]はtrue

    // 何枚選ぶかをi、選択した結果の数値をjとして配列をループ

    // jがa[i]より低いかどうかで判定
    // 低いなら、上の行の値がtrueであればしたにそのまま移植する

    // 以上なら、上の業の値がtrueか、あるいはdpの上の行のj-a[i]がtrueなら、trueにする

    // あとはYesかNoかで判定する
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
