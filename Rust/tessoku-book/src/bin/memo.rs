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

/// 二分探索を行い、リスト内に存在しているかを探す関数
fn binary_search(search_value: usize, n: usize, a: &Vec<usize>) -> bool {
    let mut l = 0;
    let mut r = n;

    while l < r {
        let m = (l + r) / 2;
        if search_value < a[m] {
            r = m
        } else if search_value > a[m] {
            l = m + 1
        } else if search_value == a[m] {
            return true;
        }
    }
    false
}

/// 直積和を求める関数
fn cartesian_sum(set1: &[usize], set2: &[usize]) -> Vec<usize> {
    let mut sums = vec![];

    for &item_a in set1 {
        for &item_b in set2 {
            sums.push(item_a + item_b);
        }
    }

    sums
}
