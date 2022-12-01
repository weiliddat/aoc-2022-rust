use std::env;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = if args.len() > 1 {
        &args[args.len() - 1]
    } else {
        panic!("Please provide a day to run!")
    };

    let day = day.as_str();

    println!("Running {}", day);

    match day {
        "day01" => day01::run(),
        _ => println!("{} was not done yet!", day),
    }
}
