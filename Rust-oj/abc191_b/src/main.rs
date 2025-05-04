use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let line1 = read_i32_vec(&mut lock);
    let _n = line1[0];
    let x = line1[1];
    let a = read_i32_vec(&mut lock);

    let v = solve(a, x);
    let output = into_string_from_i32_vec(v);

    println!("{}", output);
}

fn solve(a: Vec<i32>, x: i32) -> Vec<i32> {
    a.iter().cloned().filter(|item| *item != x).collect()
}

fn into_string_from_i32_vec(v: Vec<i32>) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_into_sring_from_i32_vec() {
        let input = vec![1, 2, 3];
        let expected = "1 2 3".to_string();

        let actual = into_string_from_i32_vec(input);

        assert_eq!(actual, expected);
    }
}
