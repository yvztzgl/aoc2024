use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut safe_levels = 0;
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let is_safe: bool = calculate_report(ip);
                if is_safe {
                    safe_levels += 1;
                }
            }
        }
    }
    println!("{}", safe_levels);
}

fn calculate_report(report: String) -> bool {
    //RULES:
    //  - The levels should be all increasing or all decreasing.
    //  - Two adjacent levels can only be differ by at least 1 and at most 3.
    //  - By those rules, n = n+1 is not valid since its not differ by at least 1.
    let mut current_level = -1;
    let mut is_increasing = false;
    let mut is_valid = true;
    let levels = report.split_whitespace();
    for (i, level) in levels.enumerate() {
        let lev = level.parse::<i32>().unwrap();
        if i == 0 {
            current_level = lev;
            continue;
        }
        // Adjacent rule n can't be equal to n+1
        if lev == current_level {
            is_valid = false;
            break;
        }
        if lev > current_level {
            if i > 1 && !is_increasing {
                is_valid = false;
                break;
            }
            is_increasing = true;
            if current_level + 3 < lev {
                is_valid = false;
                break;
            }
        } else {
            if i > 1 && is_increasing {
                is_valid = false;
                break;
            }
            if current_level - 3 > lev {
                is_valid = false;
                break;
            }
        }

        current_level = lev;
    }
    return is_valid;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
