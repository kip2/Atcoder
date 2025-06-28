use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_i64_vec(&mut reader);
    let n = line[0];
    let x = line[1];

    let mut vec = Vec::<i64>::new();

    for _ in 0..n {
        let line = read_i64_vec(&mut reader);
        let v = line[0];
        let p = line[1];
        vec.push(alcohol_content(v, p));
    }

    let ans = solve2(x, &vec);
    // let ans = solve(n, x, &mut reader);

    println!("{}", ans);
}

fn solve2(x: i64, vec: &Vec<i64>) -> i64 {
    vec.iter()
        .enumerate()
        .scan(0, |total, (i, &v)| {
            *total += v;
            Some((i + 1, *total))
        })
        .find(|&(_, total)| total > x * 100)
        .map(|(i, _)| i as i64)
        .unwrap_or(-1)
}

fn solve<R: BufRead>(n: i64, x: i64, reader: &mut R) -> i64 {
    let mut total: i64 = 0;

    for i in 1..=n {
        let line = read_i64_vec(reader);
        let v = line[0];
        let p = line[1];

        let ac = alcohol_content(v, p);
        total += ac;
        if total > x * 100 {
            return i;
        }
    }
    -1
}

fn alcohol_content(alcohol: i64, percent: i64) -> i64 {
    return alcohol * percent;
}

fn read_f64_vec<R: BufRead>(reader: &mut R) -> Vec<f64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect()
}

fn read_i64_vec<R: BufRead>(reader: &mut R) -> Vec<i64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[test]
fn test_alcohol_content() {
    let expected = 10;

    let alcohol = 200;
    let percent = 5;
    let actual = alcohol_content(alcohol, percent);

    assert_eq!(actual, expected);
}
