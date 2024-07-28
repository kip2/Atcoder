use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut friends: Vec<(usize, usize)> = vec![];

    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        friends.push((a, b));
    }

    friends.sort();

    let mut now_v = k;

    for i in 0..n {
        let f_v = friends[i].0;
        let f_m = friends[i].1;

        if f_v <= now_v {
            now_v += f_m;
        } else {
            break;
        }
    }

    println!("{}", now_v);
}
