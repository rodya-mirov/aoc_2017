use std::collections::HashMap;

const INPUT: &str = include_str!("input/8.txt");

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        IResult, Parser,
    };

    pub(crate) type RegisterName = String;
    pub(crate) type ConstantRef = i32;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(crate) struct Instruction {
        pub(crate) action: Action,
        pub(crate) condition: Condition,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(crate) struct Action {
        pub(crate) lhs: RegisterName,
        pub(crate) rhs: ConstantRef,
        pub(crate) kind: ActionType,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(crate) struct Condition {
        pub(crate) lhs: String,
        pub(crate) rhs: ConstantRef,
        pub(crate) kind: ConditionalOp,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub(crate) enum ActionType {
        Inc,
        Dec,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub(crate) enum ConditionalOp {
        Gt,
        Lt,
        Geq,
        Leq,
        Eq,
        Neq,
    }

    fn parse_line(input: &str) -> IResult<&str, Instruction> {
        use crate::lib::parse_i32 as parse_num;

        let (input, action_lhs) = take_while(char::is_alphanumeric)(input)?;

        let (input, _) = take_while(char::is_whitespace)(input)?;

        let (input, action_type): (&str, ActionType) = alt((
            tag("inc").map(|_| ActionType::Inc),
            tag("dec").map(|_| ActionType::Dec),
        ))(input)?;

        let (input, _) = take_while(char::is_whitespace)(input)?;

        let (input, action_rhs) = parse_num(input)?;

        let action = Action {
            lhs: action_lhs.to_string(),
            kind: action_type,
            rhs: action_rhs,
        };

        let (input, _) = take_while(char::is_whitespace)(input)?;
        let (input, _) = tag("if")(input)?;
        let (input, _) = take_while(char::is_whitespace)(input)?;

        let (input, cond_lhs) = take_while(char::is_alphanumeric)(input)?;

        let (input, _) = take_while(char::is_whitespace)(input)?;

        // Note: order matters, these branches are applied in order
        let (input, cond_op) = alt((
            tag(">=").map(|_| ConditionalOp::Geq),
            tag(">").map(|_| ConditionalOp::Gt),
            tag("<=").map(|_| ConditionalOp::Leq),
            tag("<").map(|_| ConditionalOp::Lt),
            tag("==").map(|_| ConditionalOp::Eq),
            tag("!=").map(|_| ConditionalOp::Neq),
        ))(input)?;

        let (input, _) = take_while(char::is_whitespace)(input)?;

        let (_input, cond_rhs) = parse_num(input)?;

        let condition = Condition {
            lhs: cond_lhs.to_string(),
            rhs: cond_rhs,
            kind: cond_op,
        };

        Ok(("", Instruction { action, condition }))
    }

    pub(crate) fn parse(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| parse_line(line).unwrap().1)
            .collect()
    }
}

use parser::{Action, ActionType, Condition, ConditionalOp};

fn eval_cond(registers: &HashMap<String, i32>, cond: Condition) -> bool {
    let lhs_val = registers.get(&cond.lhs).copied().unwrap_or(0);
    let rhs_val = cond.rhs;
    match cond.kind {
        ConditionalOp::Gt => lhs_val > rhs_val,
        ConditionalOp::Lt => lhs_val < rhs_val,
        ConditionalOp::Geq => lhs_val >= rhs_val,
        ConditionalOp::Leq => lhs_val <= rhs_val,
        ConditionalOp::Eq => lhs_val == rhs_val,
        ConditionalOp::Neq => lhs_val != rhs_val,
    }
}

fn apply_action(registers: &mut HashMap<String, i32>, action: &Action) {
    let val = registers.entry(action.lhs.clone()).or_insert(0);
    match action.kind {
        ActionType::Dec => {
            *val -= action.rhs;
        }
        ActionType::Inc => {
            *val += action.rhs;
        }
    }
}

fn run_8a_with_input(input: &str) -> i32 {
    let instructions = parser::parse(input);

    let mut registers: HashMap<String, i32> = HashMap::new();

    for instr in instructions {
        if eval_cond(&registers, instr.condition) {
            apply_action(&mut registers, &instr.action);
        }
    }

    registers.into_iter().map(|(_, v)| v).max().unwrap()
}

pub fn run_8a() -> i32 {
    run_8a_with_input(INPUT)
}

fn run_8b_with_input(input: &str) -> i32 {
    let instructions = parser::parse(input);

    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max = 0;

    for instr in instructions {
        if eval_cond(&registers, instr.condition) {
            apply_action(&mut registers, &instr.action);
            max = registers
                .get(&instr.action.lhs)
                .copied()
                .unwrap_or(0)
                .max(max);
        }
    }

    max
}

pub fn run_8b() -> i32 {
    run_8b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_8a() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        let expected = 1;
        let actual = run_8a_with_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_8b() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        let expected = 10;
        let actual = run_8b_with_input(input);

        assert_eq!(actual, expected);
    }
}
