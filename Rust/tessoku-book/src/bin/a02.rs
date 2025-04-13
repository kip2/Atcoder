fn main() {
    proconio::input! {
        N: usize,
        X: i32,
        A: [i32; N],
    }

    let mut flg = "No";

    for i in 0..N {
        if A[i] == X {
            flg = "Yes";
        }
    }

    println!("{}", flg);
}
