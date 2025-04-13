use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: usize,
        mut y: usize,
    }

    x -= 1;
    y -= 1;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for _ in 0..h {
        input! {
            line: String,
        }
        let row = line.chars().collect();
        grid.push(row);
    }

    let mut count = 1;

    for i in (x + 1)..h {
        if grid[i][y] == '#' {
            break;
        } else {
            count += 1;
        }
    }

    for i in (0..x).rev() {
        if grid[i][y] == '#' {
            break;
        } else {
            count += 1;
        }
    }

    for i in (y + 1)..w {
        if grid[x][i] == '#' {
            break;
        } else {
            count += 1;
        }
    }

    for i in (0..y).rev() {
        if grid[x][i] == '#' {
            break;
        } else {
            count += 1;
        }
    }

    println!("{}", count);
}
