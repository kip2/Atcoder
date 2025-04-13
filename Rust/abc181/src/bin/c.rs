use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();

    for _ in 0..n {
        input! {
            x_buf: i64,
            y_buf: i64,
        }

        x.push(x_buf);
        y.push(y_buf);
    }

    let mut answer = "No";

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (x[j] - x[i]) * (y[k] - y[i]) - (y[j] - y[i]) * (x[k] - x[i]) == 0 {
                    answer = "Yes";
                    break;
                }
            }
        }
    }

    println!("{}", answer);
}
