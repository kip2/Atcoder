use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut a_arr = vec![];
    let mut b_arr = vec![];
    let mut c_arr = vec![];
    let mut d_arr = vec![];

    for _ in 0..n {
        input! {
            a:usize,
            b:usize,
            c:usize,
            d:usize,
        }
        a_arr.push(a);
        b_arr.push(b);
        c_arr.push(c);
        d_arr.push(d);
    }

    let mut snowfall = vec![vec![0; w + 2]; h + 2];

    for i in 0..n {
        snowfall[a_arr[i]][b_arr[i]] += 1;
        snowfall[a_arr[i]][d_arr[i] + 1] -= 1;
        snowfall[c_arr[i] + 1][b_arr[i]] -= 1;
        snowfall[c_arr[i] + 1][d_arr[i] + 1] += 1;
    }

    for i in 1..=h {
        for j in 1..=w {
            snowfall[i][j] += snowfall[i][j - 1];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            snowfall[i][j] += snowfall[i - 1][j];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            if j >= 2 {
                print!(" ");
            }
            print!("{}", snowfall[i][j]);
        }
        println!("");
    }
}
