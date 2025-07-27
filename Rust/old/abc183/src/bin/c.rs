use itertools::{Itertools, Permutations};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut t: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        input! {
            t_buf: [usize; n],
        }

        t.push(t_buf);
    }

    // 経路のカウント用の変数を用意
    let mut answer = 0;

    // 順列を生成する
    let permutations = (0..n).permutations(n);

    // 順列でループ
    // 今いる都市、今の時間を初期化
    for perm in permutations {
        let mut now_city = 0;
        let mut now_time = 0;
        now_time += t[now_city][perm[0]];

        now_city = perm[0];

        for i in 1..n {
            let to_city = perm[i];
            now_time += t[now_city][to_city];
            now_city = to_city;
        }

        now_time += t[now_city][0];

        if now_time == k {
            answer += 1;
        }
    }

    println!("{}", answer / 2);

    // 今の都市から次の都市への時間をカウント
    // 今の年に次の歳をセット

    // 順列の残りをループする
    // 次の行き先を変数に用意
    // 今の時間にカウント
    // 今の都市を更新

    // 今の時間に、今の都市から出発点に移動する
    // 今の時間がkと一致するならanswerを増やす

    // プリント
}
