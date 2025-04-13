use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut answer: Vec<usize> = vec![];

    for i in 0..n {
        if a[i] != x {
            answer.push(a[i]);
        }
    }

    if answer.len() != 0 {
        let output = answer
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", output);
    }
}
