#![feature(is_some_and)]
use std::{env, time::Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        "71" => day7::problem1(),
        "72" => day7::problem2(),
        "81" => day8::problem1(),
        "82" => day8::problem2(),
        "91" => day9::problem1(),
        "92" => day9::problem2(),
        "101" => day10::problem1(),
        "102" => day10::problem2(),
        "111" => day11::problem1(),
        "112" => day11::problem2(),
        "121" => day12::problem1(),
        "122" => day12::problem2(),
        "131" => day13::problem1(),
        "132" => day13::problem2(),
        "141" => day14::problem1(),
        "142" => day14::problem2(),
        "151" => day15::problem1(),
        "152" => day15::problem2(),
        "161" => day16::problem1(),
        "162" => day16::problem2(),
        "171" => day17::problem1(),
        "172" => day17::problem2(),
        _ => panic!("Unkown problem"),
    };
    println!("Completed in {} ms", time.elapsed().as_millis());
}
