use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let maxes_result = find_maxes(reader);
    match maxes_result {
        Ok(maxes) => {
            println!("The top elf has {} calories", maxes[0]);
            println!(
                "The top 3 elves have {} calories in total",
                maxes[0] + maxes[1] + maxes[2]
            );
        }
        Err(_) => {
            println!("Error calculating maxes");
        }
    }
}

fn find_maxes(reader: BufReader<File>) -> Result<[i32; 3], i32> {
    let mut vec = Vec::new();
    let mut current = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    vec.push(current);
                    current = 0;
                    continue;
                }

                let parse_result = l.parse::<i32>();
                match parse_result {
                    Ok(value) => current += value,
                    Err(_) => (),
                }
            }
            Err(_) => return Err(-1),
        }
    }

    vec.sort();
    vec.reverse();

    return Ok([vec[0], vec[1], vec[2]]);
}
