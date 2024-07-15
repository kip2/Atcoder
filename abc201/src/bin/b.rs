use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vec: Vec<(String, usize)> = Vec::new();

    for _ in 0..n {
        input! {
            s: String,
            t: usize,
        }
        vec.push((s, t));
    }

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{}", &vec[vec.len() - 2].0);
}
