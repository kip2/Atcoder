use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let first_char = &s[0..1];
    let rest = &s[1..];

    println!("{}", format!("{}{}", rest, first_char));
}
