use std::io::{self, BufRead};

use itertools::Itertools;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_i32_single(&mut reader);

    let mut points = Vec::<Point>::new();

    for _ in 0..n {
        let line = read_i32_vec(&mut reader);
        let point = Point {
            x: line[0],
            y: line[1],
        };
        points.push(point);
    }

    if solve(points) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(points: Vec<Point>) -> bool {
    for combo in points.iter().combinations(3) {
        let [a, b, c] = combo.as_slice() else {
            continue;
        };
        if can_line_point_3(a, b, c) {
            return true;
        }
    }
    false
}

fn can_line_point_3(p1: &Point, p2: &Point, p3: &Point) -> bool {
    (p3.y - p1.y) * (p2.x - p1.x) - (p2.y - p1.y) * (p3.x - p1.x) == 0
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
