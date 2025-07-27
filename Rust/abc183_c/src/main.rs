use std::io::{self, BufRead};

use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_usize_vec(&mut reader);
    let n = line[0];
    let k = line[1];
    let time_table = read_usize_matrix(n, &mut reader);

    let perms = (1..n).permutations(n - 1);
    let mut ans_cnt = 0;

    for perm in perms {
        let mut total = 0;
        let mut path = vec![0];
        path.extend(&perm);
        path.extend(&vec![0]);

        for i in 0..(path.len() - 1) {
            total += time_table[path[i]][path[i + 1]];
        }
        if total == k {
            ans_cnt += 1;
        }
    }

    println!("{}", ans_cnt);
}

fn read_usize_matrix<R: BufRead>(n: usize, reader: &mut R) -> Vec<Vec<usize>> {
    let mut time_table = Vec::<Vec<usize>>::new();
    for _ in 0..n {
        let line = read_usize_vec(reader);
        time_table.push(line);
    }
    time_table
}

fn read_i64_matrix<R: BufRead>(n: i64, reader: &mut R) -> Vec<Vec<i64>> {
    let mut time_table = Vec::<Vec<i64>>::new();
    for _ in 0..n {
        let line = read_i64_vec(reader);
        time_table.push(line);
    }
    time_table
}

fn read_i32_matrix<R: BufRead>(n: i32, reader: &mut R) -> Vec<Vec<i32>> {
    let mut time_table = Vec::<Vec<i32>>::new();
    for _ in 0..n {
        let line = read_i32_vec(reader);
        time_table.push(line);
    }
    time_table
}

fn solve() -> () {}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
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

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
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
