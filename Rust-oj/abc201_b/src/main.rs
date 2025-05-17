use std::io::{self, BufRead};

struct Mountain {
    name: String,
    hight: u32,
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_i32_single(&mut reader);

    let mut first = Mountain {
        name: "nothing".to_string(),
        hight: 0,
    };

    let mut second = Mountain {
        name: "nothing".to_string(),
        hight: 0,
    };

    for _ in 0..n {
        let line = read_str_vec(&mut reader);

        let name = line[0].to_string();
        let hight = line[1].parse::<u32>().unwrap();

        if first.hight < hight {
            second = first;

            first = Mountain {
                name: name.clone(),
                hight: hight,
            }
        } else if second.hight < hight {
            second = Mountain {
                name: name.clone(),
                hight: hight,
            }
        }
    }

    println!("{}", second.name);

    // sample: get input
    // let n = read_i32_single(&mut lock);
    // let a = read_i32_vec(&mut lock);

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

fn read_str_vec<R: BufRead>(reader: &mut R) -> Vec<String> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace().map(|s| s.to_string()).collect()
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
