const INPUT: &str = include_str!("input/11.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    NE,
    SE,
    SW,
    NW,
    N,
    S,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct Totals {
    ne: i32,
    n: i32,
    se: i32,
}

impl Totals {
    fn and_dir(&mut self, dir: Dir) {
        match dir {
            Dir::NE => self.ne += 1,
            Dir::N => self.n += 1,
            Dir::SE => self.se += 1,

            Dir::SW => self.ne -= 1,
            Dir::S => self.n -= 1,
            Dir::NW => self.se -= 1,
        }
    }

    fn distance_from_origin(mut self) -> i32 {
        loop {
            let mut changed = false;

            // Basically you can replace any inefficient path with a more efficient path
            // Actual cancellations (NE+SW, SE+NW, E+W) have already been cancelled out and
            // replaced by signed direction counts.
            //      N
            //  NW      NE
            //  SW      SE
            //      S
            // Remaining inefficient paths (note SW is -NE and so on):

            //  N + SE -> NE
            while self.n > 0 && self.se > 0 {
                self.n -= 1;
                self.se -= 1;
                self.ne += 1;
                changed = true;
            }

            //  NE + S -> SE
            while self.ne > 0 && self.n < 0 {
                self.ne -= 1;
                self.n += 1;
                self.se += 1;
                changed = true;
            }

            //  SE + SW -> S
            while self.se > 0 && self.ne < 0 {
                self.se -= 1;
                self.ne += 1;
                self.n -= 1;
                changed = true;
            }

            //  S + NW -> SW
            while self.n < 0 && self.se < 0 {
                self.n += 1;
                self.se += 1;
                self.ne -= 1;
                changed = true;
            }

            //  SW + N -> NW
            while self.ne < 0 && self.n > 0 {
                self.ne += 1;
                self.n -= 1;
                self.se -= 1;
                changed = true;
            }

            //  NW + NE -> N
            while self.se < 0 && self.ne > 0 {
                self.se += 1;
                self.ne -= 1;
                self.n += 1;
                changed = true;
            }

            if !changed {
                break;
            }
        }

        self.n.abs() + self.ne.abs() + self.se.abs()
    }
}

fn parse_11a(input: &str) -> impl Iterator<Item = Dir> + '_ {
    input.split(',').map(|token| match token {
        "s" => Dir::S,
        "se" => Dir::SE,
        "ne" => Dir::NE,
        "sw" => Dir::SW,
        "n" => Dir::N,
        "nw" => Dir::NW,
        other => panic!("Unrecognized token {}", other),
    })
}

fn run_11a_with_input(input: &str) -> i32 {
    let mut totals = Totals::default();

    for dir in parse_11a(input) {
        totals.and_dir(dir);
    }

    totals.distance_from_origin()
}

pub fn run_11a() -> i32 {
    run_11a_with_input(INPUT)
}

fn run_11b_with_input(input: &str) -> i32 {
    let mut totals = Totals::default();
    let mut max = 0;

    for dir in parse_11a(input) {
        totals.and_dir(dir);
        max = max.max(totals.distance_from_origin());
    }

    max
}

pub fn run_11b() -> i32 {
    run_11b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_11a() {
        assert_eq!(run_11a_with_input("ne,ne,ne"), 3);
        assert_eq!(run_11a_with_input("ne,ne,sw,sw"), 0);
        assert_eq!(run_11a_with_input("ne,ne,s,s"), 2);
        assert_eq!(run_11a_with_input("se,n"), 1);
        assert_eq!(run_11a_with_input("se,sw,se,sw,sw"), 3);
    }
}
