const INPUT: &str = include_str!("input/2.txt");

fn run_2a_with_input(input: &str) -> u64 {
    let mut total = 0;

    for line in input.lines() {
        let mut running_min = u64::max_value();
        let mut running_max = u64::min_value();

        for token in line
            .split_whitespace()
            .map(|token| token.parse::<u64>().unwrap())
        {
            running_min = running_min.min(token);
            running_max = running_max.max(token);
        }

        total += running_max - running_min;
    }

    total
}

fn run_2b_with_input(input: &str) -> u64 {
    let mut total = 0;

    'main: for line in input.lines() {
        let nums: Vec<u64> = line.split_whitespace().map(|t| t.parse().unwrap()).collect();

        for i in 1 .. nums.len() {
            for j in 0 .. i {
                let a = nums[i];
                let b = nums[j];

                if a % b == 0 {
                    total += a / b;
                    continue 'main;
                } else if b % a == 0 {
                    total += b / a;
                    continue 'main;
                }
            }
        }

        panic!("Divisor not found in line {:?}", nums);
    }

    total
}

pub fn run_2a() -> u64 {
    run_2a_with_input(INPUT)
}

pub fn run_2b() -> u64 {
    run_2b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_a() {
        let input = "5 1 9 5 \n 7 5 3 \n 2 4 6 8 ";
        assert_eq!(run_2a_with_input(input), 18);
    }

    #[test]
    fn sample_b() {
        let input = "5 9 2 8 \n 9 4 7 3 \n 3 8 6 5";
        assert_eq!(run_2b_with_input(input), 9);
    }
}
