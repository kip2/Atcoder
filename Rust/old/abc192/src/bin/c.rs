use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = n;
    for i in 0..k {
        a = f(a);
    }

    println!("{}", a);
}

fn f(x: usize) -> usize {
    sort_digits_desc(x) - sort_digits_asc(x)
}

fn sort_digits_desc(x: usize) -> usize {
    let mut digits: Vec<char> = x.to_string().chars().collect();
    digits.sort();
    digits.reverse();

    let sorted_str: String = digits.into_iter().collect();
    sorted_str.parse::<usize>().unwrap()
}

#[test]
fn test_sort_digits_desc() {
    assert_eq!(sort_digits_desc(53421), 54321);
}

fn sort_digits_asc(x: usize) -> usize {
    let mut digits: Vec<char> = x.to_string().chars().collect();
    digits.sort();

    let sorted_str: String = digits.into_iter().collect();
    sorted_str.parse::<usize>().unwrap()
}

#[test]
fn test_sort_digits_asc() {
    assert_eq!(sort_digits_desc(53421), 12345);
}
