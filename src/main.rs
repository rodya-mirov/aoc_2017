use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: [run] [problemNumber]");
        println!("Example: cargo run --release -- 1a");
    }

    let start = std::time::Instant::now();

    let out: String = match args.get(1).unwrap().as_str() {
        "1a" => day01::run_1a().to_string(),
        "1b" => day01::run_1b().to_string(),

        "2a" => day02::run_2a().to_string(),
        "2b" => day02::run_2b().to_string(),

        "3a" => day03::run_3a().to_string(),
        "3b" => day03::run_3b().to_string(),

        "4a" => day04::run_4a().to_string(),
        "4b" => day04::run_4b().to_string(),

        "5a" => day05::run_5a().to_string(),
        "5b" => day05::run_5b().to_string(),

        "6a" => day06::run_6a().to_string(),
        "6b" => day06::run_6b().to_string(),

        "7a" => day07::run_7a().to_string(),
        "7b" => day07::run_7b().to_string(),

        "8a" => day08::run_8a().to_string(),
        "8b" => day08::run_8b().to_string(),

        "9a" => day09::run_9a().to_string(),
        "9b" => day09::run_9b().to_string(),

        "10a" => day10::run_10a().to_string(),
        "10b" => day10::run_10b().to_string(),

        "11a" => day11::run_11a().to_string(),
        "11b" => day11::run_11b().to_string(),

        "12a" => day12::run_12a().to_string(),
        "12b" => day12::run_12b().to_string(),

        "13a" => day13::run_13a().to_string(),
        "13b" => day13::run_13b().to_string(),

        "14a" => day14::run_14a().to_string(),
        "14b" => day14::run_14b().to_string(),

        "15a" => day15::run_15a().to_string(),
        "15b" => day15::run_15b().to_string(),

        "16a" => day16::run_16a().to_string(),
        "16b" => day16::run_16b().to_string(),

        "17a" => day17::run_17a().to_string(),
        "17b" => day17::run_17b().to_string(),

        other => {
            format!("Unrecognized problem number {}", other)
        }
    };

    println!("{}", out);
    println!("Process took {:.3} seconds", start.elapsed().as_secs_f32());
}
