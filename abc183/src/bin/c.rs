use itertools::Itertools;
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
    for root in permutations {
        // 今いる都市、今の時間を初期化
        let mut now_city = 0;
        let mut now_time = 0;

        // 今の都市から次の都市への時間をカウント
        // 今の年に次の歳をセット
        now_time += t[now_city][root[0]];
        now_city = root[0];

        // 順列の残りをループする
        // 次の行き先を変数に用意
        // 今の時間にカウント
        // 今の都市を更新
        for i in 1..n {
            let to_city = root[i];
            now_time += t[now_city][to_city];
            now_city = to_city;
        }

        // 今の時間に、今の都市から出発点に移動する
        now_time += t[now_city][0];

        // 今の時間がkと一致するならanswerを増やす
        if now_time == k {
            answer += 1;
        }
    }

    // プリント
    println!("{}", answer / 2);
}
