use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i32_vec(&mut reader);
    let n = line[0] as usize;
    let k = line[1] as usize;

    let p_vec = read_i32_vec(&mut reader);

    // let mut expected_vec: Vec<f64> = vec![];
    let expected_vec: Vec<f64> = p_vec
        .into_iter()
        .map(|p| calc_expected_value(p as f64))
        .collect();

    let mut pref: Vec<f64> = vec![0.0f64];
    for &ev in &expected_vec {
        pref.push(pref.last().unwrap() + ev);
    }

    let mut max_ans = 0.0 as f64;

    for i in 0..=(n - k) {
        max_ans = f64::max(max_ans, pref[i + k] - pref[i]);
    }

    println!("{:.12}", max_ans);
}

fn calc_expected_value(n: f64) -> f64 {
    (n + 1.0) / 2.0
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}
