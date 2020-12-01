use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: [run] [problemNumber]");
        println!("Example: cargo run --release -- 1a");
    }

    let start = std::time::Instant::now();

    let out: String = match args.get(1).unwrap().as_str() {
        "1a" => day1::run_1a().to_string(),
        "1b" => day1::run_1b().to_string(),

        "2a" => day2::run_2a().to_string(),
        "2b" => day2::run_2b().to_string(),

        "3a" => day3::run_3a().to_string(),
        "3b" => day3::run_3b().to_string(),

        "4a" => day4::run_4a().to_string(),
        "4b" => day4::run_4b().to_string(),

        "5a" => day5::run_5a().to_string(),
        "5b" => day5::run_5b().to_string(),

        "6a" => day6::run_6a().to_string(),
        "6b" => day6::run_6b().to_string(),

        "7a" => day7::run_7a().to_string(),
        "7b" => day7::run_7b().to_string(),

        "8a" => day8::run_8a().to_string(),
        "8b" => day8::run_8b().to_string(),

        other => {
            format!("Unrecognized problem number {}", other)
        }
    };

    println!("{}", out);
    println!("Process took {:.3} seconds", start.elapsed().as_secs_f32());
}
