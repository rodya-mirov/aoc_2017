use std::collections::HashMap;

const INPUT: &str = include_str!("input/23.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum DataRef {
    Reg(char),
    Const(i64),
}

impl std::fmt::Debug for DataRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataRef::Reg(c) => {
                write!(f, "'{}'", c)
            }
            DataRef::Const(c) => {
                write!(f, "{}", c)
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Op {
    Set(char, DataRef),
    Sub(char, DataRef),
    Mul(char, DataRef),
    Jnz(DataRef, i64),
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum AST {
    Set(char, Expr),
    Sub(char, Expr),
    Mul(char, Expr),
    DoWhileNonzero { dp: Expr, code: Vec<AST> },
    Loop { code: Vec<AST> },
    IfZero { dp: Expr, code: Vec<AST> },
    IfNonZero { dp: Expr, code: Vec<AST> },
    Stop,
}

#[derive(Clone, Eq, PartialEq)]
enum Expr {
    Const(i64),
    Reg(char),
}

impl Into<Expr> for char {
    fn into(self) -> Expr {
        Expr::Reg(self)
    }
}

impl Into<Expr> for DataRef {
    fn into(self) -> Expr {
        match self {
            DataRef::Reg(c) => Expr::Reg(c),
            DataRef::Const(i) => Expr::Const(i),
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Reg(c) => {
                write!(f, "'{}'", c)
            }
            Expr::Const(i) => {
                write!(f, "{}", i)
            }
        }
    }
}

mod parse {
    use super::{DataRef, Op};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, char as exact},
        combinator::{eof, map},
        multi::fold_many1,
        sequence::tuple,
        IResult,
    };

    use crate::lib::parse_i64;

    fn parse_space(input: &str) -> IResult<&str, ()> {
        fold_many1(exact(' '), (), |_, _| ())(input)
    }

    fn parse_dataref(input: &str) -> IResult<&str, DataRef> {
        alt((
            map(parse_i64, |i| DataRef::Const(i)),
            map(anychar, |c| DataRef::Reg(c)),
        ))(input)
    }

    fn parse_instr(input: &str) -> IResult<&str, Op> {
        map(
            tuple((
                alt((
                    map(
                        tuple((tag("set"), parse_space, anychar, parse_space, parse_dataref)),
                        |(_, _, c, _, d)| Op::Set(c, d),
                    ),
                    map(
                        tuple((tag("sub"), parse_space, anychar, parse_space, parse_dataref)),
                        |(_, _, c, _, d)| Op::Sub(c, d),
                    ),
                    map(
                        tuple((tag("mul"), parse_space, anychar, parse_space, parse_dataref)),
                        |(_, _, c, _, d)| Op::Mul(c, d),
                    ),
                    map(
                        tuple((
                            tag("jnz"),
                            parse_space,
                            parse_dataref,
                            parse_space,
                            parse_i64,
                        )),
                        |(_, _, c, _, d)| Op::Jnz(c, d),
                    ),
                )),
                eof,
            )),
            |(op, _)| op,
        )(input)
    }

    pub(super) fn parse(input: &str) -> Vec<Op> {
        let mut ops = Vec::new();
        for line in input.lines() {
            let (_, op) = parse_instr(line).unwrap();
            ops.push(op);
        }
        ops
    }
}

mod to_ast {
    use std::collections::{HashMap, HashSet};

    use super::{DataRef, Op, AST};

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum SimpleTrans {
        Set(char, DataRef),
        Sub(char, DataRef),
        Mul(char, DataRef),
        Goto(usize),
        Stop,
        GotoNZ(char, usize),
        StopNZ(char),
    }

    impl Into<PartialTrans> for SimpleTrans {
        fn into(self) -> PartialTrans {
            PartialTrans::Simple(self)
        }
    }

    impl Into<PartialTrans> for AST {
        fn into(self) -> PartialTrans {
            PartialTrans::Full(self)
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    enum PartialTrans {
        Simple(SimpleTrans),
        Full(AST),
    }

    impl std::fmt::Debug for PartialTrans {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PartialTrans::Simple(s) => {
                    write!(f, "{:?}", s)
                }
                PartialTrans::Full(s) => {
                    write!(f, "{:?}", s)
                }
            }
        }
    }

    fn is_finished(op: &PartialTrans) -> bool {
        match op {
            PartialTrans::Full(_) => true,
            PartialTrans::Simple(_) => false,
        }
    }

    fn print_ops(
        simple_trans: &[(usize, PartialTrans)],
        label_refs: &HashMap<usize, HashSet<usize>>,
    ) {
        for (ip, code) in simple_trans.iter() {
            if label_refs.contains_key(ip) {
                println!("Label {}:", ip);
            }
            println!("  {}: {:?}", ip, code);
        }

        println!("Label refs: {:?}", label_refs);
        println!();
    }

    pub(super) fn to_ast(ops: &[Op]) -> Vec<AST> {
        let mut simple_trans: Vec<(usize, PartialTrans)> = Vec::new();

        let mut label_refs = HashMap::new();

        for (ip, op) in ops.iter().copied().enumerate() {
            match op {
                Op::Set(r, d) => {
                    simple_trans.push((ip, SimpleTrans::Set(r, d).into()));
                }
                Op::Sub(r, d) => {
                    simple_trans.push((ip, SimpleTrans::Sub(r, d).into()));
                }
                Op::Mul(r, d) => {
                    simple_trans.push((ip, SimpleTrans::Mul(r, d).into()));
                }
                Op::Jnz(r, d) => {
                    match r {
                        DataRef::Reg(r) => {
                            if ip as i64 + d >= ops.len() as i64 {
                                simple_trans.push((ip, SimpleTrans::StopNZ(r).into()));
                            } else {
                                let label = (ip as i64 + d) as usize;
                                simple_trans.push((ip, SimpleTrans::GotoNZ(r, label).into()));
                                label_refs.entry(label).or_insert(HashSet::new()).insert(ip);
                            }
                        }
                        DataRef::Const(c) => {
                            if c == 0 {
                                // nothing; but i don't think this ever happens
                            } else {
                                if ip as i64 + d >= ops.len() as i64 {
                                    simple_trans.push((ip, SimpleTrans::Stop.into()));
                                } else {
                                    let label = (ip as i64 + d) as usize;
                                    simple_trans.push((ip, SimpleTrans::Goto(label).into()));
                                    label_refs.entry(label).or_insert(HashSet::new()).insert(ip);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Anything "simple" should be transformed into a fully compiled AST right away
        for (_, op) in simple_trans.iter_mut() {
            compile_easy(op);
        }

        print_ops(&simple_trans, &label_refs);

        while collapse_pure_goto(&mut simple_trans, &mut label_refs)
            || remove_negative_branch(&mut simple_trans, &mut label_refs)
        {
            print_ops(&simple_trans, &label_refs);
        }

        if simple_trans.iter().all(|(_, op)| is_finished(op)) {
            return simple_trans
                .into_iter()
                .map(|(_, op)| match op {
                    PartialTrans::Simple(_) => unreachable!(),
                    PartialTrans::Full(f) => f,
                })
                .collect();
        } else {
            println!("Optimization incomplete");

            print_ops(&simple_trans, &label_refs);

            unimplemented!()
        }
    }

    /// Attempts to identify a "pure" jump that can be compiled away
    /// Returns true if any change was made
    fn collapse_pure_goto(
        ops: &mut Vec<(usize, PartialTrans)>,
        label_refs: &mut HashMap<usize, HashSet<usize>>,
    ) -> bool {
        let mut change_made = false;

        for (&label, refs) in label_refs.iter() {
            // for now, just skip it
            if refs.len() != 1 {
                continue;
            }

            // PRE: nothing of length
            let r = refs.iter().copied().next().unwrap();

            if label == r {
                unimplemented!()
            } else if label < r {
                // then potentially an "dowhilenz ..."
                let pure = ops
                    .iter()
                    .filter(|(ip, _)| label <= *ip && *ip < r)
                    .all(|(_, op)| is_finished(op));
                if pure {
                    change_made = true;

                    println!("Collapsing label {} to ref {}", label, r);

                    let old = std::mem::replace(ops, Vec::new());
                    let mut inner_code = Vec::new();

                    for (ip, op) in old {
                        if ip < label {
                            ops.push((ip, op));
                        } else if ip < r {
                            inner_code.push(op);
                        } else if ip == r {
                            let inner = std::mem::replace(&mut inner_code, Vec::new());
                            let inner = inner
                                .into_iter()
                                .map(|op| match op {
                                    PartialTrans::Simple(_) => unreachable!(),
                                    PartialTrans::Full(ast) => ast,
                                })
                                .collect();

                            match op {
                                PartialTrans::Simple(SimpleTrans::Goto(_)) => {
                                    ops.push((ip, AST::Loop { code: inner }.into()));
                                }
                                PartialTrans::Simple(SimpleTrans::GotoNZ(c, _)) => {
                                    ops.push((
                                        ip,
                                        AST::DoWhileNonzero {
                                            dp: c.into(),
                                            code: inner,
                                        }
                                        .into(),
                                    ));
                                }
                                _ => {
                                    unimplemented!("Cannot collapse from {:?}", op);
                                }
                            }
                        } else {
                            ops.push((ip, op));
                        }
                    }

                    clear_refs(label_refs, &[r]);
                    break;
                }
            } else {
                // then potentially a "ifzero ..."
                let pure = ops
                    .iter()
                    .filter(|(ip, _)| r < *ip && *ip < label)
                    .all(|(_, op)| is_finished(op));

                if pure {
                    change_made = true;

                    println!("Collapsing label {} to ref {}", label, r);

                    let old = std::mem::replace(ops, Vec::new());
                    let mut inner_code = Vec::new();
                    // invalid, will be replaced
                    let mut op_replacement: Box<dyn Fn(Vec<AST>) -> AST> =
                        Box::new(|_| AST::Loop { code: Vec::new() });

                    for (ip, op) in old {
                        if ip < r {
                            ops.push((ip, op));
                        } else if ip == r {
                            // grab the dp and type
                            match op {
                                PartialTrans::Simple(s) => match s {
                                    SimpleTrans::Goto(_) => {
                                        op_replacement = Box::new(|code| AST::Loop { code });
                                    }
                                    SimpleTrans::GotoNZ(c, _) => {
                                        op_replacement =
                                            Box::new(move |code| AST::IfZero { dp: c.into(), code })
                                    }
                                    _ => unreachable!(),
                                },
                                PartialTrans::Full(_) => unreachable!(),
                            }
                        } else if ip < label {
                            inner_code.push(op);
                        } else if ip == label {
                            let inner = std::mem::replace(&mut inner_code, Vec::new());
                            let inner = inner
                                .into_iter()
                                .map(|op| match op {
                                    PartialTrans::Simple(_) => unreachable!(),
                                    PartialTrans::Full(ast) => ast,
                                })
                                .collect();

                            ops.push((r, op_replacement(inner).into()));
                            ops.push((ip, op));
                        } else {
                            ops.push((ip, op));
                        }
                    }

                    clear_refs(label_refs, &[r]);
                    break;
                }
            }
        }

        change_made
    }

    fn remove_negative_branch(
        ops: &mut Vec<(usize, PartialTrans)>,
        label_refs: &mut HashMap<usize, HashSet<usize>>,
    ) -> bool {
        let mut saved: Option<(usize, usize, usize, char)> = None;

        'bigloop: for (i, (ip_1, op_1)) in ops.iter().enumerate().skip(1) {
            if let PartialTrans::Simple(SimpleTrans::Goto(label)) = op_1 {
                let (ip_0, op_0) = ops.get(i - 1).unwrap();
                if let PartialTrans::Simple(SimpleTrans::GotoNZ(c, skip_label)) = op_0 {
                    if let Some((ip_2, _)) = ops.get(i + 1) {
                        if skip_label == ip_2 {
                            let inner_commands = ops
                                .iter()
                                .filter(|(ip, _)| ip_1 < ip && ip < label)
                                .collect::<Vec<&(usize, PartialTrans)>>();

                            if inner_commands.iter().all(|(_, c)| is_finished(c)) {
                                let mut all_good = true;
                                for (ip, _) in &inner_commands {
                                    if let Some(s) = label_refs.get(ip) {
                                        if s != &(vec![ip_0].into_iter().copied().collect()) {
                                            println!("Inner label refs for {}: {:?}", ip, s);
                                            all_good = false;
                                        }
                                    }
                                }

                                if all_good {
                                    saved = Some((*ip_0, *ip_1, *label, *c));
                                    break 'bigloop;
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some((ip_0, ip_1, label, c)) = saved {
            let mut code = Vec::new();

            let old = std::mem::replace(ops, Vec::new());
            for (old_ip, old_op) in old {
                if old_ip < ip_0 {
                    ops.push((old_ip, old_op));
                } else if old_ip <= ip_1 {
                    // skip
                } else if old_ip < label {
                    code.push(match old_op {
                        PartialTrans::Simple(_) => unreachable!(),
                        PartialTrans::Full(ast) => ast,
                    });
                } else if old_ip == label {
                    let my_code = std::mem::replace(&mut code, Vec::new());
                    let new_command = AST::IfNonZero {
                        dp: c.into(),
                        code: my_code,
                    };
                    ops.push((ip_0, new_command.into()));
                    ops.push((old_ip, old_op));
                } else {
                    ops.push((old_ip, old_op));
                }
            }

            clear_refs(label_refs, &[ip_0, ip_1]);

            println!(
                "I think I found one: remove {} to {} and then ip {} should be the inside",
                ip_0, label, label
            );

            true
        } else {
            false
        }
    }

    fn clear_refs(label_refs: &mut HashMap<usize, HashSet<usize>>, removed_gotos: &[usize]) {
        let mut empty = Vec::new();
        for (label, refs) in label_refs.iter_mut() {
            for g in removed_gotos.iter() {
                refs.remove(g);
            }
            if refs.is_empty() {
                empty.push(*label);
            }
        }
        for label in empty {
            label_refs.remove(&label);
        }
    }

    /// Replaces any "simple" ops with their full versions. No need to repeat this.
    fn compile_easy(op: &mut PartialTrans) {
        let old = std::mem::replace(op, PartialTrans::Full(AST::Stop)); // whatever
        *op = match old {
            PartialTrans::Simple(s) => match s {
                SimpleTrans::Set(c, d) => AST::Set(c, d.into()).into(),
                SimpleTrans::Sub(c, d) => AST::Sub(c, d.into()).into(),
                SimpleTrans::Mul(c, d) => AST::Mul(c, d.into()).into(),
                SimpleTrans::Goto(ip) => SimpleTrans::Goto(ip).into(),
                SimpleTrans::Stop => AST::Stop.into(),
                SimpleTrans::GotoNZ(c, d) => SimpleTrans::GotoNZ(c, d).into(),
                SimpleTrans::StopNZ(c) => SimpleTrans::StopNZ(c).into(),
            },
            PartialTrans::Full(f) => PartialTrans::Full(f),
        }
    }
}

fn resolve(data: &HashMap<char, i64>, d: DataRef) -> i64 {
    match d {
        DataRef::Reg(r) => data.get(&r).copied().unwrap_or(0),
        DataRef::Const(x) => x,
    }
}

fn run_23a_with_input(input: &str) -> usize {
    let ops = parse::parse(input);

    let mut ip: isize = 0;
    let mut data: HashMap<char, i64> = HashMap::new();

    let mut muls = 0;

    while ip < ops.len() as isize {
        match ops[ip as usize] {
            Op::Set(c, d) => {
                let d = resolve(&data, d);
                *data.entry(c).or_insert(0) = d;
            }
            Op::Sub(c, d) => {
                let d = resolve(&data, d);
                *data.entry(c).or_insert(0) -= d;
            }
            Op::Mul(c, d) => {
                let d = resolve(&data, d);
                *data.entry(c).or_insert(0) *= d;
                muls += 1;
            }
            Op::Jnz(x, y) => {
                let x = resolve(&data, x);
                if x != 0 {
                    // gonna get the other +1 at the end
                    ip += y as isize - 1;
                }
            }
        }
        ip += 1;
    }

    muls
}

pub fn run_23a() -> usize {
    run_23a_with_input(INPUT)
}

fn run_23b_with_input(input: &str) -> i64 {
    let ops = parse::parse(input);

    let mut ast = to_ast::to_ast(&ops);
    ast.insert(0, AST::Set('a', Expr::Const(1)));

    println!("Before optimization:");
    for a in &ast {
        println!("{:?}", a);
    }
    println!();

    // I hand-optimized this, it turns out to be equivalent to this
    // but I don't know how, in good conscience, to call that a "code solution" for arbitrary
    // input, because I don't know what other kinds of input there might be
    /*
    b = 109900
    c = 126900

    loop {
        if b is composite {
            h += 1
        }

        if b - c == 0 {
            STOP
        }

        b += 17
    }
     */

    // just counted primes in that weird range with this lame python script

    //  def is_prime(b):
    //      if b < 2:
    // 	 	    return False
    //   	if b % 2 == 0:
    // 	    	return b == 2
    //  	p = 3
    // 	    while p*p < b:
    // 		    if b % p == 0:
    // 			    return False
    //      	p += 2
    //  	if p < b and b % p == 0:
    // 	    	return False
    //  	return True
    // (b, c) = (109900, 126900)
    // len([x for x in range(b, c+17, 17) if not is_prime(x)])
    913
}

pub fn run_23b() -> i64 {
    run_23b_with_input(INPUT)
}
