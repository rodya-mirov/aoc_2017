const INPUT: &str = include_str!("input/1.txt");

fn run_1a_with_input(input_str: &str) -> u64 {
    let digits: Vec<u32> = input_str.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut total: u64 = 0;
    for i in 1..digits.len() {
        if digits[i] == digits[i - 1] {
            total += digits[i] as u64;
        }
    }

    if digits[digits.len() - 1] == digits[0] {
        total += digits[0] as u64;
    }

    total
}

fn run_1b_with_input(input_str: &str) -> u64 {
    let digits: Vec<u32> = input_str.chars().map(|c| c.to_digit(10).unwrap()).collect();

    if digits.len() % 2 != 0 {
        panic!("Cannot handle a list of length {}", digits.len());
    }

    let half_length = digits.len() / 2;

    let mut total: u64 = 0;
    for i in 0 .. half_length {
        if digits[i] == digits[i + half_length] {
            total += (digits[i] as u64) * 2;
        }
    }

    total
}

pub fn run_1a() -> u64 {
    run_1a_with_input(INPUT)
}

pub fn run_1b() -> u64 {
    run_1b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1a() {
        assert_eq!(run_1a_with_input("1122"), 3)
    }

    #[test]
    fn sample_1b() {
        assert_eq!(run_1b_with_input("1122"), 0)
    }

    #[test]
    fn sample_1b_2() {
        assert_eq!(run_1b_with_input("1212"), 6)
    }
}
