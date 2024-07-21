fn main() {
    println!("Hello, world!");
}

/// 10進数を8進数に変換する関数
/// usize -> usize に対応
///
fn decimal_to_octal(n: usize) -> usize {
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
    assert_eq!(decimal_to_octal(1), 1);
    assert_eq!(decimal_to_octal(83), 123);
}

/// 数字を受け取り、昇順に並べ直して数字として返す関数
///
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

/// 数字を受け取り、降順に並べ直して数字として返す関数
///
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

/// 2次元のchar型の配列から、一列の文字列を作成する
///
/// xで指定した行に対して行う
///
fn get_row_as_string(chat: &Vec<Vec<&str>>, x: usize) -> String {
    chat[x].join("")
}

#[test]
fn test_get_row_as_string() {
    let chat = vec![
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
    ];

    assert_eq!(get_row_as_string(&chat, 1), "def");
}

/// 2次元のchar型の配列から、一列の文字列を作成する
///
/// yで指定した列に対して行う
///
fn get_column_as_string(chat: &Vec<Vec<&str>>, y: usize) -> String {
    chat.iter()
        .map(|row| row[y])
        .collect::<Vec<&str>>()
        .join("")
}

#[test]
fn test_get_column_as_string() {
    let chat = vec![
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
    ];

    assert_eq!(get_column_as_string(&chat, 2), "cfi");
}
