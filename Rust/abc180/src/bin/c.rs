use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = vec![];

    for i in 1..=(n as f64).sqrt() as usize {
        if n % i == 0 {
            answer.push(i);
            if i != n / i {
                answer.push(n / i);
            }
        }
    }
    answer.sort();

    for i in 0..answer.len() {
        println!("{}", answer[i]);
    }
}
