use std::{env, time::Instant};

mod day1;
mod day2;

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
        _ => panic!("Unkown problem"),
    };
    println!("Completed in {} ms", time.elapsed().as_millis());
}
