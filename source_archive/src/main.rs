fn main() {
    println!("Hello, world!");
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
