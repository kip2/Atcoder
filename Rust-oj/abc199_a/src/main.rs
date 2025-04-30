use std::io::{self, BufRead};

fn main() {
    let input = read_i32_vec();

    let a = input[0];
    let b = input[1];
    let c = input[2];

    let result = solve(a, b, c);

    println!("{}", result);
}

fn solve(a: i32, b: i32, c: i32) -> String {
    if a * a + b * b < c * c {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn read_i32_vec() -> Vec<i32> {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    read_i32_vec_from(&mut lock)
}

fn read_i32_vec_from<R: BufRead>(reader: &mut R) -> Vec<i32> {
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
    fn test_read_i32_vec_from() {
        let input = b"10 20 30\n";
        let mut cursor = Cursor::new(&input[..]);

        let result = read_i32_vec_from(&mut cursor);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_read_i32_vec_from_with_extra_spaces() {
        let input = b" 7 8 9 \n";
        let mut cursor = Cursor::new(&input[..]);

        let result = read_i32_vec_from(&mut cursor);
        assert_eq!(result, vec![7, 8, 9]);
    }
}
