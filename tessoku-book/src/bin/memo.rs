use proconio::input;
use superslice::Ext;

fn main() {
    // 単純な受け取り
    input! {
        n: usize,
    }

    // 配列の受け取り
    input! {
        n: usize,
        a: [usize; n],
    }

    // mutな配列の受け取り
    input! {
        n: usize,
        mut a: [usize; n],
    }

    // 配列のsort
    a.sort();

    // 0を含めたvectorを受け取る方法
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut a_with_zero = vec![0];
    a_with_zero.extend(&a);

    // lower_bound
    // supersliceクレートを使用する
    // use superslice::Ext;
    a.lower_bound(&6);
}
