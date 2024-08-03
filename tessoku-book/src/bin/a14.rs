use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    input! {
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let mut ab = cartesian_sum(&a, &b);
    let mut cd = cartesian_sum(&c, &d);

    ab.sort();
    cd.sort();

    let mut answer = "No";

    for item_ab in ab {
        let expected = k - item_ab;
        if binary_search(expected, cd.len(), &cd) {
            answer = "Yes";
            break;
        }
    }

    println!("{}", answer);
}

fn binary_search(x: usize, n: usize, a: &Vec<usize>) -> bool {
    let mut l = 0;
    let mut r = n;

    while l < r {
        let m = (l + r) / 2;
        if x < a[m] {
            r = m
        } else if x > a[m] {
            l = m + 1
        } else if x == a[m] {
            return true;
        }
    }
    false
}

fn cartesian_sum(set1: &[usize], set2: &[usize]) -> Vec<usize> {
    let mut sums = vec![];

    for &item_a in set1 {
        for &item_b in set2 {
            sums.push(item_a + item_b);
        }
    }

    sums
}
