use std::io::{self, BufRead};

#[derive(Debug)]
enum State {
    Start,
    A,
    AB,
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let _n = read_i32_single(&mut reader);
    let s = read_string(&mut reader);

    let mut count = 0;
    let mut state = State::Start;

    for c in s.chars() {
        state = match (state, c) {
            (State::Start, 'A') => State::A,
            (_, 'A') => State::A,

            (State::A, 'B') => State::AB,
            (_, 'B') => State::Start,

            (State::AB, 'C') => {
                count += 1;
                State::Start
            }
            (_, 'C') => State::Start,

            _ => State::Start,
        };
    }

    println!("{}", count);
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
