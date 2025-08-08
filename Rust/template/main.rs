use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // sample: get input
    // let n = read_i32_single(&mut reader);
    // let a = read_i32_vec(&mut reader);

    // println!("{:?}", n);
    // println!("{:?}", a);

    // output: solved value
    // let value = solve(n, a);
    // println!("{:?}", value);
}

fn solve() -> () {}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_u64_single<R: BufRead>(reader: &mut R) -> u64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<u64>().unwrap()
}

fn read_u64_vec<R: BufRead>(reader: &mut R) -> Vec<u64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn into_string_from_u64_vec(v: Vec<u64>) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
fn read_i64_single<R: BufRead>(reader: &mut R) -> i64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i64>().unwrap()
}

fn read_i64_vec<R: BufRead>(reader: &mut R) -> Vec<i64> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn into_string_from_i64_vec(v: Vec<i64>) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn read_i32_single<R: BufRead>(reader: &mut R) -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}

fn read_i32_vec<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn into_string_from_i32_vec(v: Vec<i32>) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn read_usize_single<R: BufRead>(reader: &mut R) -> usize {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<usize>().unwrap()
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn read_i128_single<R: BufRead>(reader: &mut R) -> i128 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i128>().unwrap()
}

fn read_i128_vec<R: BufRead>(reader: &mut R) -> Vec<i128> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}
fn read_u128_single<R: BufRead>(reader: &mut R) -> u128 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<u128>().unwrap()
}

fn read_u128_vec<R: BufRead>(reader: &mut R) -> Vec<u128> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
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

    #[test]
    fn test_read_string() {
        let input = "abc";
        let mut cursor = Cursor::new(&input[..]);
        let actual = read_string(&mut cursor);

        assert_eq!(actual, input);
    }

    #[test]
    fn test_read_i32_vec_from() {
        let input = b"10 20 30\n";
        let mut cursor = Cursor::new(&input[..]);

        let result = read_i32_vec(&mut cursor);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_read_i32_vec_from_with_extra_spaces() {
        let input = b" 7 8 9 \n";
        let mut cursor = Cursor::new(&input[..]);

        let result = read_i32_vec(&mut cursor);
        assert_eq!(result, vec![7, 8, 9]);
    }
}
