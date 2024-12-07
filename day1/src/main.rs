use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left_locations: Vec<i32> = Vec::new();
    let mut righ_locations: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let locations = ip.split_whitespace();
                for (i, loc) in locations.enumerate() {
                    if i == 0 {
                        left_locations.push(loc.parse::<i32>().unwrap());
                    } else {
                        righ_locations.push(loc.parse::<i32>().unwrap());
                    }
                }
            }
        }
    }
    //TODO: Make sorting in the array push above ^^. 
    //      (ex: While appending to the list, calculate its size according 
    //      to the numbers on the list and push it in the correct index, thus sorting on the go!);
    left_locations.sort();
    righ_locations.sort();

    let mut totalDistance = 0;
    for i in 0..left_locations.len() {
        totalDistance += (left_locations[i] - righ_locations[i]).abs()
    }
    println!("{}", totalDistance)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
