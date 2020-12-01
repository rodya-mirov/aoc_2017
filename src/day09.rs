const INPUT: &str = include_str!("input/9.txt");

mod parser {
    use nom::multi::fold_many0;
    use nom::{
        branch::alt,
        character::complete::{anychar, char, none_of},
        multi::separated_list0,
        sequence::tuple,
        IResult, Parser,
    };

    #[derive(Clone, Eq, PartialEq, Debug)]
    pub(crate) struct Group {
        pub(crate) garbage: Vec<Garbage>,
        pub(crate) children: Vec<Group>,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub(crate) struct Garbage {
        pub contents: usize,
    }

    fn parse_garbage(input: &str) -> IResult<&str, Garbage> {
        let mut parser = tuple((
            // Start with open angle
            char('<'),
            // Any sequence of stuff that doesn't end the garbage block
            // Note we are capturing the quantity of garbage
            fold_many0(
                alt((
                    // escape inside garbage (contents skipped)
                    tuple((char('!'), anychar)).map(|_| 0),
                    // or just whatever isn't a close angle (contents counted)
                    none_of(">").map(|_| 1),
                )),
                // add em up
                0,
                |acc, item| acc + item,
            ),
            // End with close angle
            char('>'),
        ));

        let (input, (_, contents, _)) = parser(input)?;

        std::mem::drop(parser);

        Ok((input, Garbage { contents }))
    }

    enum GroupOrGarbage {
        Group(Group),
        Garbage(Garbage),
    }

    fn parse_group_or_garbage(input: &str) -> IResult<&str, GroupOrGarbage> {
        alt((
            parse_garbage.map(|g| GroupOrGarbage::Garbage(g)),
            parse_group.map(|g| GroupOrGarbage::Group(g)),
        ))(input)
    }

    fn parse_group(input: &str) -> IResult<&str, Group> {
        let out = tuple((
            char('{'),
            separated_list0(char(','), parse_group_or_garbage),
            char('}'),
        ))(input);

        out.map(|(input, output)| {
            let (_, subgroups, _) = output;
            let mut children = vec![];
            let mut garbage = vec![];

            for child in subgroups {
                match child {
                    GroupOrGarbage::Group(g) => children.push(g),
                    GroupOrGarbage::Garbage(g) => garbage.push(g),
                }
            }

            (input, Group { children, garbage })
        })
    }

    pub(crate) fn parse(input: &str) -> Group {
        let (leftover, group) = parse_group(input).unwrap();
        if !leftover.is_empty() {
            panic!("Did not parse entire input; leftover: '{}'", leftover);
        }

        group
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_garbage() {
            assert_eq!(parse_garbage("<!!>"), Ok(("", Garbage { contents: 0 })));
            assert_eq!(parse_garbage("<a>"), Ok(("", Garbage { contents: 1 })));
        }

        #[test]
        fn test_parse_list() {
            assert_eq!(
                parse_group("{<>,<>}"),
                Ok((
                    "",
                    Group {
                        garbage: vec![Garbage { contents: 0 }, Garbage { contents: 0 }],
                        children: vec![]
                    }
                ))
            );
        }
    }
}

use parser::{parse, Group};

fn run_9a_with_input(input: &str) -> i32 {
    let top = parse(input);

    fn dfs(gp: &Group, depth: i32) -> i32 {
        depth
            + gp.children
                .iter()
                .map(|child| dfs(child, depth + 1))
                .sum::<i32>()
    }

    dfs(&top, 1)
}

pub fn run_9a() -> i32 {
    run_9a_with_input(INPUT)
}

fn run_9b_with_input(input: &str) -> usize {
    let top = parse(input);

    fn dfs(gp: &Group) -> usize {
        gp.garbage.iter().map(|g| g.contents).sum::<usize>()
            + gp.children.iter().map(dfs).sum::<usize>()
    }

    dfs(&top)
}

pub fn run_9b() -> usize {
    run_9b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_9a() {
        let samples: Vec<(&str, i32)> = vec![
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];

        for (input, expected) in samples {
            let actual = run_9a_with_input(input);
            assert_eq!(expected, actual, "{}", input);
        }
    }

    #[test]
    fn sample_9b() {
        let samples: Vec<(&str, usize)> = vec![
            ("{<>}", 0),
            ("{{},{<random characters>}}", 17),
            ("{<<<<>}", 3),
            ("{<{!>}>}", 2),
            ("{{{<!!>}},{}}", 0),
            ("{<!!!>>}", 0),
            ("{<{o\"i!a,<{i<a>}", 10),
        ];

        for (input, expected) in samples {
            let actual = run_9b_with_input(input);
            assert_eq!(expected, actual, "{}", input);
        }
    }
}
