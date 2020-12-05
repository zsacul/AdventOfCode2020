mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod tools;

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
    */
    let day5_data  = tools::read_1d_string("data/day05.txt");
    day05::solve(&day5_data);

// todo generic reader table_1d<T> table_2d<T>
}
