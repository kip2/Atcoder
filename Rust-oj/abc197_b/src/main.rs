use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let values = read_usize_vec(&mut reader);
    let (h, w, x, y) = (values[0], values[1], values[2], values[3]);

    let mut map: Vec<Vec<String>> = Vec::new();

    for _ in 0..h {
        let s = read_string_to_vec(&mut reader);
        map.push(s);
    }

    let count = solve(h, w, x - 1, y - 1, &map);

    println!("{}", count);
}

fn solve(h: usize, w: usize, x: usize, y: usize, map: &Vec<Vec<String>>) -> usize {
    let mut count = 0;

    if map[x][y] == "." {
        count += 1;
    }

    // x-の方向
    for i in (0..y).rev() {
        if map[x][i] == "#" {
            break;
        } else if map[x][i] == "." {
            count += 1;
        }
    }

    // x+の方向
    for i in y + 1..w {
        if map[x][i] == "#" {
            break;
        } else if map[x][i] == "." {
            count += 1;
        }
    }

    // y-の方向
    for i in (0..x).rev() {
        if map[i][y] == "#" {
            break;
        } else if map[i][y] == "." {
            count += 1;
        }
    }

    // y+の方向
    for i in x + 1..h {
        if map[i][y] == "#" {
            break;
        } else if map[i][y] == "." {
            count += 1;
        }
    }

    count
}

fn read_string_to_vec<R: BufRead>(reader: &mut R) -> Vec<String> {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input
        .trim()
        .to_string()
        .chars()
        .map(|c| c.to_string())
        .collect()
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
