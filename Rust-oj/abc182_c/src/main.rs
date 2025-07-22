use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_string(&mut reader);
    let k = n.len();

    let numbers = to_u8_vec(&n);
    let remainders = to_remainder_vec(&numbers);
    let remainder_sum = sum_remainder_of_u8_vec(&numbers);

    let answer = match remainder_sum {
        0 => "0",
        1 => check_case(&remainders, k, 1),
        2 => check_case(&remainders, k, 2),
        _ => "0",
    };

    println!("{}", answer);
}

fn check_case(remainders: &Vec<u8>, k: usize, target: u8) -> &'static str {
    let has_target = match target {
        1 => has_remaider_1(remainders),
        2 => has_remaider_2(remainders),
        _ => false,
    };

    if has_target {
        if k <= 1 {
            "-1"
        } else {
            "1"
        }
    } else {
        if k <= 2 {
            "-1"
        } else {
            "2"
        }
    }
}

fn has_remaider_2(nums: &Vec<u8>) -> bool {
    nums.iter().any(|&x| x == 2)
}

fn has_remaider_1(nums: &Vec<u8>) -> bool {
    nums.iter().any(|&x| x == 1)
}

fn sum_remainder_of_u8_vec(nums: &Vec<u8>) -> u8 {
    let sum: u8 = nums.iter().map(|n| n % 3).sum();
    sum % 3
}

fn to_remainder_vec(nums: &Vec<u8>) -> Vec<u8> {
    let remainders: Vec<u8> = nums.iter().map(|n| n % 3).collect();
    remainders
}

fn to_u8_vec(s: &str) -> Vec<u8> {
    let digits: Vec<u8> = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    digits
}

#[test]
fn has_remaider_2_test() {
    let numbers: Vec<u8> = vec![1, 2, 0, 1, 2];
    let expected = true;
    let actual = has_remaider_2(&numbers);

    assert_eq!(actual, expected);

    let numbers: Vec<u8> = vec![0, 1, 0, 0, 1];
    let expected = false;
    let actual = has_remaider_2(&numbers);

    assert_eq!(actual, expected);
}

#[test]
fn has_remaider_1_test() {
    let numbers: Vec<u8> = vec![1, 2, 0, 1, 2];
    let expected = true;
    let actual = has_remaider_1(&numbers);

    assert_eq!(actual, expected);

    let numbers: Vec<u8> = vec![0, 2, 0, 0, 2];
    let expected = false;
    let actual = has_remaider_1(&numbers);

    assert_eq!(actual, expected);
}

#[test]
fn sum_remainder_of_u8_vec_test() {
    let numbers: Vec<u8> = vec![1, 2, 3, 4, 5];
    let expected = 6 as u8;
    let actual = sum_remainder_of_u8_vec(&numbers);

    assert_eq!(expected, actual);
}

#[test]
fn to_u8_vec_test() {
    let s = String::from("1234567890");

    let actual = to_u8_vec(&s);
    let expected: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    assert_eq!(expected, actual);
}

fn read_string<R: BufRead>(reader: &mut R) -> String {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
