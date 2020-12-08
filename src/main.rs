mod tools;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
/*
    let day1_data    = tools::read_1d_i32("data/day01.txt");
    day01::solve(&day1_data);

    let day2_data  = tools::read_1d_string("data/day02.txt");
    day02::solve(&day2_data);

    let day3_data  = tools::read_1d_string("data/day03.txt");
    day03::solve(&day3_data);

    let day4_data  = tools::read_1d_string("data/day04.txt");
    day04::solve(&day4_data);
    
    let day5_data  = tools::read_1d_string("data/day05.txt");
    day05::solve(&day5_data);
    
    let day6_data  = tools::read_1d_string("data/day06.txt");
    day06::solve(&day6_data);

    let day7_data  = tools::read_1d_string("data/day07.txt");
    day07::solve(&day7_data);
*/    

    let day8_data  = tools::read_1d_string("data/day08.txt");
    day08::solve(&day8_data);

// todo generic reader table_1d<T> table_2d<T>
}
