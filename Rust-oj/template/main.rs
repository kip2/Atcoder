use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    // sample: get input
    // let n = read_i32_single(&mut lock);
    // let a = read_i32_vec(&mut lock);

    // println!("{:?}", n);
    // println!("{:?}", a);

    println!("{:?}", nums);
}

fn solve() -> () {}

fn read_string() -> String {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    read_string_from(&mut lock)
}

fn read_string_from<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
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
    fn test_read_string() {
        let input = "abc";
        let mut cursor = Cursor::new(&input[..]);
        let actual = read_string_from(&mut cursor);

        assert_eq!(actual, input);
    }

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
