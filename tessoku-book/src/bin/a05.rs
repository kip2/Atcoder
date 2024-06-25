fn main() {
    proconio::input! {
        n: i32,
        k: i32,
    }

    let mut cnt = 0;

    for i in 1..=n {
        for j in 1..=n {
            let buf = k - (i + j);
            if 1 <= buf && buf <= n {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
