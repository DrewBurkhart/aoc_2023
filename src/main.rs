use std::{env, time::Instant};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let problem = env::args()
        .nth(1)
        .expect("Please provide problem number as {day}{problem} i.e. 11");
    get_solution(problem);
}

fn get_solution(problem: String) {
    let time = Instant::now();
    match problem.as_str() {
        "11" => day1::problem1(),
        "12" => day1::problem2(),
        "21" => day2::problem1(),
        "22" => day2::problem2(),
        "31" => day3::problem1(),
        "32" => day3::problem2(),
        "41" => day4::problem1(),
        "42" => day4::problem2(),
        "51" => day5::problem1(),
        "52" => day5::problem2(),
        "61" => day6::problem1(),
        "62" => day6::problem2(),
        _ => panic!("Unkown problem"),
    };
    println!("Completed in {} ms", time.elapsed().as_millis());
}
