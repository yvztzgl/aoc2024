use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // Search for mul( (4 chars.)
                for i in 0..ip.len() {
                    if i + 4 > ip.len() {
                        break;
                    }
                    let subset = &ip[i..i + 4];
                    if subset == "mul(" {
                        total += check_for_mul(ip.clone(), i + 4);
                        //println!("i:{} subset: {}",i,subset);
                    }
                }
            }
        }
    }
    println!("{}",total);
}

fn check_for_mul(line: String, index: usize) -> i32 {
    let mut mult_total = 0;
    let mut comma_found = false;
    let mut bracelet_found = false;
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut current = index - 1;
    let mut is_valid = true;
    while !(comma_found && bracelet_found) {
        current += 1;
        if line.as_bytes()[current] == b',' {
            //println!("COMMA FOUND AT: {}", current);
            comma_found = true;
            continue;
        }
        if line.as_bytes()[current] == b')' {
            //println!("BRACELET FOUND AT: {}", current);
            bracelet_found = true;
            break;
        }
        if line.chars().nth(current).unwrap() == ' ' {
            is_valid = false;
            break;
        }
        if comma_found {
            y += &line.chars().nth(current).unwrap().to_string();
        } else {
            x += &line.chars().nth(current).unwrap().to_string();
        }
    }
    if is_valid && comma_found && bracelet_found {
        if !x.parse::<i32>().is_err() && !y.parse::<i32>().is_err(){
            let x_actual = x.parse::<i32>().unwrap();
            let y_actual = y.parse::<i32>().unwrap();
            mult_total = x_actual * y_actual;
        }
    }

    return mult_total;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
