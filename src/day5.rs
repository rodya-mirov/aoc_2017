const INPUT: &str = include_str!("input/5.txt");

#[derive(Clone, Eq, PartialEq)]
struct VM {
    ip: isize,
    jumps: Vec<isize>,
}

impl VM {
    fn new(jumps: Vec<isize>) -> VM {
        Self { ip: 0, jumps }
    }

    fn update_5a(&mut self) -> Result<(), ()> {
        if self.ip < 0 {
            return Err(());
        }

        let this_ip = self.ip as usize;
        if this_ip >= self.jumps.len() {
            return Err(());
        }

        self.ip += self.jumps[this_ip];
        self.jumps[this_ip] += 1;

        Ok(())
    }

    fn update_5b(&mut self) -> Result<(), ()> {
        if self.ip < 0 {
            return Err(());
        }

        let jump_trans = |old| {
            if old >= 3 {
                old - 1
            } else {
                old + 1
            }
        };

        let this_ip = self.ip as usize;
        if this_ip >= self.jumps.len() {
            return Err(());
        }

        self.ip += self.jumps[this_ip];
        self.jumps[this_ip] = jump_trans(self.jumps[this_ip]);

        Ok(())
    }

    fn run_until_dead_5a(mut self) -> usize {
        let mut jumps = 0;

        while let Ok(_) = self.update_5a() {
            jumps += 1;
        }

        jumps
    }

    fn run_until_dead_5b(mut self) -> usize {
        let mut jumps = 0;

        while let Ok(_) = self.update_5b() {
            jumps += 1;
        }

        jumps
    }
}

fn run_5a_with_inputs(jumps: &str) -> usize {
    let jumps: Vec<isize> = jumps
        .split_whitespace()
        .map(|token| token.parse().unwrap())
        .collect();

    let vm = VM::new(jumps);

    vm.run_until_dead_5a()
}

pub fn run_5a() -> usize {
    run_5a_with_inputs(INPUT)
}

fn run_5b_with_inputs(jumps: &str) -> usize {
    let jumps: Vec<isize> = jumps
        .split_whitespace()
        .map(|token| token.parse().unwrap())
        .collect();

    let vm = VM::new(jumps);

    vm.run_until_dead_5b()
}

pub fn run_5b() -> usize {
    run_5b_with_inputs(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_5a() {
        assert_eq!(run_5a_with_inputs("0 3 0 1 -3"), 5);
    }

    #[test]
    fn sample_5b() {
        assert_eq!(run_5b_with_inputs("0 3 0 1 -3"), 10);
    }
}
