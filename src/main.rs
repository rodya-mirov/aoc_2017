use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: [run] [problemNumber]");
        println!("Example: cargo run --release -- 1a");
    }

    let out: String = match args.get(1).unwrap().as_str() {
        "1a" => day1::run_1a().to_string(),
        "1b" => day1::run_1b().to_string(),

        "2a" => day2::run_2a().to_string(),
        "2b" => day2::run_2b().to_string(),

        other => {
            format!("Unrecognized problem number {}", other)
        }
    };

    println!("{}", out);
}
