use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_i32_single(&mut reader);

    let mut set: HashSet<String> = HashSet::new();

    for _ in 0..n {
        let s = read_string(&mut reader);
        set.insert(s);
    }

    println!("{}", set.len());
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
