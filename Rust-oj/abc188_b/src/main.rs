use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let _n = read_i32_single(&mut lock);
    let a = read_i32_vec(&mut lock);
    let b = read_i32_vec(&mut lock);

    let result = solve(a, b);
    println!("{}", result)
}

fn solve(a: Vec<i32>, b: Vec<i32>) -> String {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    if sum == 0 {
        return "Yes".to_string();
    }

    return "No".to_string();
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
