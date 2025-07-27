fn main() {
    proconio::input! {
        n: i32,
    }

    for i in (0..=9).rev() {
        let wari = 1 << i;
        print!("{}", n / wari % 2);
    }
}
