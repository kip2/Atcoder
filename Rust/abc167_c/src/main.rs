use std::{
    cmp::min,
    io::{self, BufRead},
    usize,
};

#[derive(Debug)]
struct Book {
    cost: usize,
    algorithm_comprehension: Vec<usize>,
}
impl Book {
    fn new(list: Vec<usize>) -> Self {
        Self {
            cost: list[0],
            algorithm_comprehension: list[1..].to_vec(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line = read_usize_vec(&mut reader);
    let n = line[0];
    let m = line[1];
    let x = line[2];

    let books = create_books(&mut reader, n);

    let mut min_cost: usize = usize::MAX;

    for mask in 0..(1 << n) {
        let result = (0..n)
            // filter bit set iterator
            .filter(|&i| is_bit_set(mask, i))
            // map filtered books
            .map(|i| &books[i])
            // Struct of accumulator is (x, v)
            // x : sum costs of buy books.
            // v : vector is algorithm comprehension of buy books.
            .fold((0, vec![0; m]), |(mut cost_sum, mut acc), book| {
                cost_sum += book.cost;
                for (a, v) in acc.iter_mut().zip(&book.algorithm_comprehension) {
                    *a += v;
                }
                (cost_sum, acc)
            });

        let exists = result.1.iter().any(|&y| y < x);

        if !exists {
            min_cost = min(min_cost, result.0);
        }
    }

    match min_cost {
        usize::MAX => println!("{}", -1),
        _ => println!("{}", min_cost),
    }
}

fn is_bit_set(mask: usize, i: usize) -> bool {
    (mask & (1_usize << i)) != 0
}

fn create_books<R: BufRead>(reader: &mut R, n: usize) -> Vec<Book> {
    let mut books: Vec<Book> = vec![];
    for _ in 0..n {
        let line = read_usize_vec(reader);
        let book = Book::new(line);
        books.push(book);
    }
    books
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
