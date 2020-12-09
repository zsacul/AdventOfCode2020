use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//pub fn read_1d<T: FromStr + Num>(path:String)->Vec<T>
#[allow(unused)]
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

#[allow(unused)]
pub fn read_1d_i64(path:&str)->Vec<i64>
{
    let mut res:Vec<i64> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                res.push(ip.parse::<i64>().unwrap());
            }
        }
    }
    res
}


#[allow(unused)]
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
