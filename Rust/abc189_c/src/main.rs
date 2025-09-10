use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Combination {
    l: usize,
    r: usize,
    x: usize,
}

impl Combination {
    fn total(&self) -> usize {
        (self.r - self.l + 1) * self.x
    }
}

impl Eq for Combination {}

impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total().partial_cmp(&other.total())
    }
}

impl Ord for Combination {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total().cmp(&other.total())
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_usize_single(&mut reader);
    let a = read_usize_vec(&mut reader);

    let mut answer = Combination { l: 0, r: 0, x: 0 };

    for l in 0..n {
        let mut mn = a[l];
        for r in l..n {
            mn = mn.min(a[r]);
            let combination = Combination { l: l, r: r, x: mn };
            if answer < combination {
                answer = combination;
            }
        }
    }
    println!("{}", answer.total());
}

fn read_usize_single<R: BufRead>(reader: &mut R) -> usize {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<usize>().unwrap()
}

fn read_usize_vec<R: BufRead>(reader: &mut R) -> Vec<usize> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[test]
fn total_of_Combination_test() {
    let comb1 = Combination { l: 1, r: 2, x: 3 };
    let comb2 = Combination { l: 2, r: 3, x: 3 };

    assert_eq!(comb1 == comb2, true);
    assert_eq!(comb1 > comb2, false);
    assert_eq!(comb1 < comb2, false);

    let comb1 = Combination { l: 1, r: 10, x: 2 };
    let comb2 = Combination { l: 2, r: 10, x: 2 };

    assert_eq!(comb1 == comb2, false);
    assert_eq!(comb1 > comb2, true);
    assert_eq!(comb1 < comb2, false);

    let comb1 = Combination { l: 2, r: 10, x: 2 };
    let comb2 = Combination { l: 1, r: 10, x: 2 };

    assert_eq!(comb1 == comb2, false);
    assert_eq!(comb1 > comb2, false);
    assert_eq!(comb1 < comb2, true);

    let comb1 = Combination { l: 2, r: 20, x: 2 };
    let comb2 = Combination { l: 2, r: 10, x: 2 };

    assert_eq!(comb1 == comb2, false);
    assert_eq!(comb1 > comb2, true);
    assert_eq!(comb1 < comb2, false);

    let comb1 = Combination { l: 2, r: 10, x: 3 };
    let comb2 = Combination { l: 2, r: 10, x: 2 };

    assert_eq!(comb1 == comb2, false);
    assert_eq!(comb1 > comb2, true);
    assert_eq!(comb1 < comb2, false);

    let comb1 = Combination { l: 2, r: 10, x: 2 };
    let comb2 = Combination { l: 2, r: 10, x: 3 };

    assert_eq!(comb1 == comb2, false);
    assert_eq!(comb1 > comb2, false);
    assert_eq!(comb1 < comb2, true);

    // 0 case
    let answer = Combination { l: 0, r: 0, x: 0 };
    println!("{}", answer.total());
}
