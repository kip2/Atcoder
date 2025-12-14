fn main() {
    println!("Hello, world!");
}

/*
   &str -> String
*/
fn from_str_to_string(s: &str) -> String {
    s.to_owned()
}

/*
    &String -> String
*/
fn from_string_ref_to_string(s: &String) -> String {
    s.clone()
}

/*
    Vec<char> -> String
*/
fn from_chars_to_string(chars: Vec<char>) -> String {
    chars.iter().collect()
}

/*
   Vec<String> -> String
*/
fn from_vec_string_to_string(v: Vec<String>) -> String {
    v.join(" ")
}

/*
   usize -> String
*/
fn from_usize_to_string(n: usize) -> String {
    n.to_string()
}

/*
   Vec<char> -> String
*/
fn conv_vec_char_to_string(chars: Vec<char>) -> String {
    chars.into_iter().collect()
}

/*
   &String -> Vec<char>
*/
fn from_string_to_chars(s: &String) -> Vec<char> {
    s.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_to_string() {
        let s = "hello";
        let result = from_str_to_string(s);
        assert_eq!(result, "hello".to_string());
    }

    #[test]
    fn test_from_string_ref_to_string() {
        let s = String::from("world");
        let result = from_string_ref_to_string(&s);
        assert_eq!(result, "world".to_string());
        // clone なので元の String は消えていない
        assert_eq!(s, "world".to_string());
    }

    #[test]
    fn test_from_chars_to_string() {
        let chars = vec!['R', 'u', 's', 't'];
        let result = from_chars_to_string(chars);
        assert_eq!(result, "Rust".to_string());
    }

    #[test]
    fn test_from_vec_string_to_string() {
        let v = vec!["hello".to_string(), "rust".to_string(), "world".to_string()];
        let result = from_vec_string_to_string(v);
        assert_eq!(result, "hello rust world".to_string());
    }

    #[test]
    fn test_from_usize_to_string() {
        let n = 12345_usize;
        let result = from_usize_to_string(n);
        assert_eq!(result, "12345".to_string());
    }

    #[test]
    fn test_conv_vec_char_to_string() {
        let chars = vec!['🦀', 'R', 'u', 's', 't'];
        let result = conv_vec_char_to_string(chars);
        assert_eq!(result, "🦀Rust".to_string());
    }

    #[test]
    fn test_from_string_to_chars() {
        let s = String::from("abc🦀");
        let result = from_string_to_chars(&s);
        assert_eq!(result, vec!['a', 'b', 'c', '🦀']);
    }
}
