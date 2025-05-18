use std::io::{self, BufRead};

struct Mountain {
    name: String,
    hight: u32,
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let n = read_i32_single(&mut reader);

    let mut first = Mountain {
        name: "nothing".to_string(),
        hight: 0,
    };

    let mut second = Mountain {
        name: "nothing".to_string(),
        hight: 0,
    };

    for _ in 0..n {
        let line = read_str_vec(&mut reader);

        let name = line[0].to_string();
        let hight = line[1].parse::<u32>().unwrap();

        if first.hight < hight {
            second = first;

            first = Mountain {
                name: name.clone(),
                hight: hight,
            }
        } else if second.hight < hight {
            second = Mountain {
                name: name.clone(),
                hight: hight,
            }
        }
    }

    println!("{}", second.name);
}

fn read_str_vec<R: BufRead>(reader: &mut R) -> Vec<String> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn read_i32_single<R: BufRead>(reader: &mut R) -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse::<i32>().unwrap()
}
