use proconio::input;

fn main() {
    // input
    input! {
        h: usize,
        w: usize,
    }

    let mut squares = vec![];
    for _ in 0..h {
        input! {
            x: [usize; w],
        }
        squares.push(x);
    }

    input! {
        q: usize,
    }

    let mut a_arr = vec![];
    let mut b_arr = vec![];
    let mut c_arr = vec![];
    let mut d_arr = vec![];

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        a_arr.push(a);
        b_arr.push(b);
        c_arr.push(c);
        d_arr.push(d);
    }

    // resolution code
    // calculating cumulative sum
    let mut cumulative_sum = vec![vec![0; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            cumulative_sum[i + 1][j + 1] = cumulative_sum[i + 1][j] + squares[i][j];
        }
    }

    for i in 0..w {
        for j in 0..h {
            cumulative_sum[j + 1][i + 1] += cumulative_sum[j][i + 1];
        }
    }

    // print ansqwer
    for i in 0..q {
        let answer = cumulative_sum[c_arr[i]][d_arr[i]]
            - cumulative_sum[c_arr[i]][b_arr[i] - 1]
            - cumulative_sum[a_arr[i] - 1][d_arr[i]]
            + cumulative_sum[a_arr[i] - 1][b_arr[i] - 1];
        println!("{}", answer);
    }
}
