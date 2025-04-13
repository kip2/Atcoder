use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut count = 0;

    for decimal in 1..=n {
        let octal = decimal_to_octal(decimal);
        if include_seven(decimal) || include_seven(octal) {
            count += 1;
        }
    }

    println!("{}", n - count);
}

fn include_seven(n: usize) -> bool {
    n.to_string().contains("7")
}

fn decimal_to_octal(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut division = n;
    let mut result: Vec<String> = vec![];
    while division != 0 {
        result.push((division % 8).to_string());
        division /= 8;
    }

    result.reverse();

    result
        .concat()
        .parse::<usize>()
        .expect("Failed to parse string to usize")
}

#[test]
fn test_decimal_to_octal() {
    assert_eq!(decimal_to_octal(0), 0);
    assert_eq!(decimal_to_octal(1), 1);
    assert_eq!(decimal_to_octal(83), 123);
}
