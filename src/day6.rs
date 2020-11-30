use std::collections::{HashMap, HashSet};

const INPUT: &str = "11	11	13	7	0	15	5	5	4	4	1	1	7	1	15	11";

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct VM {
    blocks: Vec<u32>,
}

impl VM {
    fn step(&mut self) {
        let len = self.blocks.len();

        let mut best_ind = 0;
        let mut best_weight = 0;

        for i in 0..len {
            let this_weight = self.blocks[i];
            if this_weight > best_weight {
                best_weight = this_weight;
                best_ind = i;
            }
        }

        self.blocks[best_ind] = 0;

        while best_weight > 0 {
            best_ind += 1;
            if best_ind >= len {
                best_ind = 0;
            }
            self.blocks[best_ind] += 1;
            best_weight -= 1;
        }
    }
}

fn run_6a_with_input(input: &str) -> usize {
    let blocks = input
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    let mut vm = VM { blocks };

    let mut seen = HashSet::new();
    while seen.insert(vm.clone()) {
        vm.step();
    }

    seen.len()
}

fn run_6b_with_input(input: &str) -> usize {
    let blocks = input
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    let mut vm = VM { blocks };

    let mut seen = HashMap::new();

    loop {
        let old = seen.insert(vm.clone(), seen.len());
        if let Some(old_steps) = old {
            return seen.len() - old_steps;
        }
        vm.step();
    }
}

pub fn run_6a() -> usize {
    run_6a_with_input(INPUT)
}

pub fn run_6b() -> usize {
    run_6b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_6a() {
        assert_eq!(run_6a_with_input("0 2 7 0"), 5);
    }

    #[test]
    fn sample_6b() {
        assert_eq!(run_6b_with_input("0 2 7 0"), 4);
    }
}
