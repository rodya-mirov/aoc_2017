use std::collections::HashMap;
use std::collections::VecDeque;

const INPUT: &str = include_str!("input/18.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cmd {
    Snd(char),
    Set(char, DataRef),
    Add(char, DataRef),
    Mul(char, DataRef),
    Mod(char, DataRef),
    Rcv(char),
    Jgz(DataRef, DataRef),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DataRef {
    Reg(char),
    Val(i64),
}

mod parse {
    use super::{Cmd, DataRef};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, char as exact_char, digit1, satisfy},
        combinator::{eof, map},
        sequence::tuple,
        IResult,
    };

    fn parse_num(input: &str) -> IResult<&str, i64> {
        alt((
            map(digit1, |s: &str| s.parse::<i64>().unwrap()),
            map(tuple((exact_char('-'), digit1)), |(_, s): (_, &str)| {
                -(s.parse::<i64>().unwrap())
            }),
        ))(input)
    }

    fn parse_reg(input: &str) -> IResult<&str, char> {
        satisfy(char::is_alphabetic)(input)
    }

    fn parse_ref(input: &str) -> IResult<&str, DataRef> {
        alt((
            map(parse_num, |n| DataRef::Val(n)),
            map(parse_reg, |c| DataRef::Reg(c)),
        ))(input)
    }

    fn parse_snd(input: &str) -> IResult<&str, Cmd> {
        map(tuple((tag("snd "), anychar)), |(_, c)| Cmd::Snd(c))(input)
    }

    fn parse_set(input: &str) -> IResult<&str, Cmd> {
        map(
            tuple((tag("set "), anychar, exact_char(' '), parse_ref)),
            |(_, c, _, i)| Cmd::Set(c, i),
        )(input)
    }

    fn parse_add(input: &str) -> IResult<&str, Cmd> {
        map(
            tuple((tag("add "), anychar, exact_char(' '), parse_ref)),
            |(_, c, _, i)| Cmd::Add(c, i),
        )(input)
    }

    fn parse_mul(input: &str) -> IResult<&str, Cmd> {
        map(
            tuple((tag("mul "), anychar, exact_char(' '), parse_ref)),
            |(_, c, _, i)| Cmd::Mul(c, i),
        )(input)
    }

    fn parse_mod(input: &str) -> IResult<&str, Cmd> {
        map(
            tuple((tag("mod "), anychar, exact_char(' '), parse_ref)),
            |(_, c, _, i)| Cmd::Mod(c, i),
        )(input)
    }

    fn parse_rcv(input: &str) -> IResult<&str, Cmd> {
        map(tuple((tag("rcv "), anychar)), |(_, c)| Cmd::Rcv(c))(input)
    }

    fn parse_jgz(input: &str) -> IResult<&str, Cmd> {
        map(
            tuple((tag("jgz "), parse_ref, exact_char(' '), parse_ref)),
            |(_, c, _, i)| Cmd::Jgz(c, i),
        )(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Cmd> {
        map(
            tuple((
                alt((
                    parse_snd, parse_set, parse_add, parse_mul, parse_mod, parse_rcv, parse_jgz,
                )),
                eof,
            )),
            |(cmd, _)| cmd,
        )(input)
    }

    pub(super) fn parse(input: &str) -> Vec<Cmd> {
        let mut out = Vec::new();
        for line in input.lines() {
            let (_, cmd) = parse_line(line).unwrap();
            out.push(cmd);
        }
        out
    }
}

struct VmA {
    ip: i64,
    code: Vec<Cmd>,
    data: HashMap<char, i64>,
    recent_sound: Option<i64>,
}

impl VmA {
    fn new(code: Vec<Cmd>) -> Self {
        VmA {
            ip: 0,
            code,
            data: HashMap::new(),
            recent_sound: None,
        }
    }

    fn data(&mut self, c: char) -> &mut i64 {
        self.data.entry(c).or_insert(0)
    }

    fn resolve(&mut self, r: DataRef) -> i64 {
        match r {
            DataRef::Val(v) => v,
            DataRef::Reg(c) => *self.data(c),
        }
    }

    // Run code. Return Some(snd) if there was a recover operation.
    // Return None if not. If a recover was run but no sound has been played,
    // then it will panic (oops).
    fn run(&mut self) -> i64 {
        let ip_max = self.code.len() as i64;

        loop {
            if self.ip < 0 || self.ip >= ip_max {
                panic!("OOB {}", self.ip);
            }
            match self.code[self.ip as usize] {
                Cmd::Snd(x) => {
                    let snd = *self.data(x);
                    self.recent_sound = Some(snd);
                }
                Cmd::Set(x, y) => {
                    let y = self.resolve(y);
                    self.data.insert(x, y);
                }
                Cmd::Add(x, y) => {
                    let y = self.resolve(y);
                    *self.data(x) += y;
                }
                Cmd::Mul(x, y) => {
                    let y = self.resolve(y);
                    *self.data(x) *= y;
                }
                Cmd::Mod(x, y) => {
                    let y = self.resolve(y);
                    *self.data(x) %= y;
                }
                Cmd::Rcv(x) => {
                    if *self.data(x) != 0 {
                        return self
                            .recent_sound
                            .expect("Should have a recently played sound");
                    }
                }
                Cmd::Jgz(x, y) => {
                    let x = self.resolve(x);
                    if x > 0 {
                        let y = self.resolve(y);
                        self.ip += y - 1;
                        if y == 0 {
                            panic!("Infinite loop, stupid jump");
                        }
                    }
                }
            }

            self.ip += 1;
        }
    }
}

fn run_18a_with_input(input: &str) -> i64 {
    let code = parse::parse(input);
    let mut vm = VmA::new(code);
    vm.run()
}

pub fn run_18a() -> i64 {
    run_18a_with_input(INPUT)
}

struct VmB {
    ip: i64,
    code: Vec<Cmd>,
    data: HashMap<char, i64>,
}

impl VmB {
    fn new(code: Vec<Cmd>, p_code: i64) -> Self {
        let mut data = HashMap::new();
        data.insert('p', p_code);
        VmB { ip: 0, code, data }
    }

    fn data(&mut self, c: char) -> &mut i64 {
        self.data.entry(c).or_insert(0)
    }

    fn resolve(&mut self, r: DataRef) -> i64 {
        match r {
            DataRef::Val(v) => v,
            DataRef::Reg(c) => *self.data(c),
        }
    }

    // Run code. Continues until it needs to do a receive and that fails.
    fn run<R, S>(&mut self, mut rcv: R, mut snd: S)
    where
        R: FnMut() -> Option<i64>,
        S: FnMut(i64),
    {
        let ip_max = self.code.len() as i64;

        loop {
            if self.ip < 0 || self.ip >= ip_max {
                panic!("OOB {}", self.ip);
            }
            match self.code[self.ip as usize] {
                Cmd::Snd(x) => {
                    let snd_data = *self.data(x);
                    snd(snd_data);
                }
                Cmd::Set(x, y) => {
                    let y = self.resolve(y);
                    self.data.insert(x, y);
                }
                Cmd::Add(x, y) => {
                    let y = self.resolve(y);
                    *self.data(x) += y;
                }
                Cmd::Mul(x, y) => {
                    let y = self.resolve(y);
                    *self.data(x) *= y;
                }
                Cmd::Mod(x, y) => {
                    let y = self.resolve(y);
                    *self.data(x) %= y;
                }
                Cmd::Rcv(x) => {
                    let rcv_data = rcv();
                    match rcv_data {
                        None => return,
                        Some(data) => {
                            *self.data(x) = data;
                        }
                    }
                }
                Cmd::Jgz(x, y) => {
                    let x = self.resolve(x);
                    if x > 0 {
                        let y = self.resolve(y);
                        self.ip += y - 1;
                        if y == 0 {
                            panic!("Infinite loop, stupid jump");
                        }
                    }
                }
            }

            self.ip += 1;
        }
    }
}

fn run_18b_with_input(input: &str) -> usize {
    let code = parse::parse(input);

    let mut vm0 = VmB::new(code.clone(), 0);
    let mut vm1 = VmB::new(code, 1);

    let mut queue_0: VecDeque<i64> = VecDeque::new();
    let mut queue_1: VecDeque<i64> = VecDeque::new();

    let mut vm0_sends: usize = 0;
    let mut vm1_sends: usize = 0;

    loop {
        let last_sends = (vm0_sends, vm1_sends);

        vm0.run(
            || queue_0.pop_front(),
            |dat| {
                vm0_sends += 1;
                queue_1.push_back(dat);
            },
        );

        vm1.run(
            || queue_1.pop_front(),
            |dat| {
                vm1_sends += 1;
                queue_0.push_back(dat);
            },
        );

        if (vm0_sends, vm1_sends) == last_sends {
            break;
        }
    }

    vm1_sends
}

pub fn run_18b() -> usize {
    run_18b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_18a() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

        assert_eq!(run_18a_with_input(input), 4);
    }

    #[test]
    fn sample_18b() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

        assert_eq!(run_18b_with_input(input), 3);
    }
}
