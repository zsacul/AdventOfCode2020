use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day01;
mod day02;
mod day03;

//pub fn read_1d<T: FromStr + Num>(path:String)->Vec<T>
pub fn read_1d_i32(path:&str)->Vec<i32>
{
    let mut res:Vec<i32> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                res.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    res
}

pub fn read_1d_string(path:&str)->Vec<String>
{
    let mut res:Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                res.push(ip);
            }
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let day1_data    = read_1d_i32("../data/day01.txt");
    day01::solve(&day1_data);

    let day2_data  = read_1d_string("../data/day02.txt");
    day02::solve(&day2_data);

    let day3_data  = read_1d_string("../data/day03.txt");
    day03::solve(&day3_data);

// todo generic reader table_1d<T> table_2d<T>
}
