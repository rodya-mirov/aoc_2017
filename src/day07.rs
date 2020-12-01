use std::collections::HashMap;

const INPUT: &str = include_str!("input/7.txt");

#[derive(Debug)]
struct ParseLine {
    name: String,
    weight: u32,
    holding: Vec<String>,
}

struct Tree {
    name: String,
    weight: u32,
    children: Vec<Tree>,
}

/// Nom nom nom
fn parse_line(line: &str) -> nom::IResult<&str, ParseLine> {
    use nom::{
        bytes::complete::{is_not, tag, take_while},
        character::complete::char as exact_char,
        error::{Error, ErrorKind},
        multi::separated_list1,
        sequence::delimited,
    };

    let (input, name) = take_while(char::is_alphanumeric)(line)?;

    let (input, _) = take_while(char::is_whitespace)(input)?;

    let (input, weight): (&str, &str) =
        delimited(exact_char('('), is_not(")"), exact_char(')'))(input)?;

    let holding: Vec<String>;

    if !input.is_empty() {
        let (input, _) = take_while(char::is_whitespace)(input)?;
        let (input, _) = nom::bytes::complete::tag("->")(input)?;
        let (input, _) = take_while(char::is_whitespace)(input)?;

        let (input, list) = separated_list1(tag(", "), is_not(","))(input)?;

        if !input.is_empty() {
            return Err(nom::Err::Failure(Error::new(input, ErrorKind::NonEmpty)));
        }
        holding = list.into_iter().map(|s| s.to_string()).collect();
    } else {
        holding = Vec::new();
    }

    Ok((
        input,
        ParseLine {
            name: name.to_string(),
            weight: weight.parse().unwrap(),
            holding,
        },
    ))
}

fn parse_tree(input: &str) -> Tree {
    // Map parentId -> list<childId>
    let mut children_relns: HashMap<String, Vec<String>> = HashMap::new();

    // Trees with correct weight but empty child lists; this will be populated later
    let mut to_process: Vec<Tree> = Vec::new();

    for line in input.lines() {
        // input is guaranteed to be valid so unwrap is fine
        let (_, parsed) = parse_line(line).unwrap();

        to_process.push(Tree {
            name: parsed.name.clone(),
            weight: parsed.weight,
            children: Vec::new(),
        });

        children_relns.insert(parsed.name, parsed.holding);
    }

    // Simplified topological sort since we know it's a tree (no cycles, no multiple parents):
    // repeat until "to process" is empty:
    //      for each node remaining to be processed:
    //          for each child in the list, if it's "dead" (moved to leaves) then:
    //              delete it from the children-to-process list
    //              pull the child tree out of the leaves list and add it to the parent tree's child list
    //          if there are now no remaining children, it's a leaf; move it to the leaves list
    // runtime O(n^2) which is ... fine

    let mut leaves: HashMap<String, Tree> = HashMap::new();

    while !to_process.is_empty() {
        let mut next = Vec::new();
        for mut node in to_process {
            let children_to_process: &mut Vec<String> = children_relns.get_mut(&node.name).unwrap();

            let old_children = std::mem::replace(children_to_process, Vec::new());

            for child_name in old_children {
                if let Some(leaf) = leaves.remove(&child_name) {
                    node.children.push(leaf);
                } else {
                    children_to_process.push(child_name);
                }
            }

            // Then we're a leaf
            if children_to_process.is_empty() {
                leaves.insert(node.name.clone(), node);
            } else {
                next.push(node);
            }
        }
        to_process = next;
    }

    assert_eq!(leaves.len(), 1, "Should only be one leaf node at the end");

    leaves.into_iter().map(|(_, v)| v).next().unwrap()
}

fn run_7a_with_input(input: &str) -> String {
    parse_tree(input).name
}

pub fn run_7a() -> String {
    run_7a_with_input(INPUT)
}

fn run_7b_with_input(input: &str) -> u32 {
    let tree = parse_tree(input);

    // Basically the idea is this; we recursively search
    // The use of Result is janky but it makes the code SO much shorter because we can use ?
    // Basically Ok(w) means "everything was fine and this is the total weight of the node"
    // Err(w) means "something was the wrong weight and this is what it should be"
    fn dfs(tree: &Tree) -> Result<u32, u32> {
        let num_children = tree.children.len();

        match num_children {
            0 => Ok(tree.weight),
            1 => Ok(tree.weight + dfs(&tree.children[0])?),
            num_children => {
                let mut child_weights: Vec<u32> = Vec::with_capacity(num_children);
                let mut child_weight_set: HashMap<u32, usize> = HashMap::new(); // weight -> count of that weight
                for child in &tree.children {
                    let child_weight = dfs(child)?;
                    child_weights.push(child_weight);
                    *child_weight_set.entry(child_weight).or_insert(0) += 1;
                }

                if child_weight_set.len() == 1 {
                    Ok(tree.weight + child_weights[0] * (num_children as u32))
                } else if num_children == 2 {
                    panic!("Bad input: if there are two children and they have differing weights, the solution is ambiguous");
                } else {
                    // We assume a good weight exists
                    let desired_weight = child_weight_set
                        .iter()
                        .filter(|(_, v)| **v != 1)
                        .map(|(k, _)| *k)
                        .next()
                        .unwrap();

                    for (child_ind, child_weight) in child_weights.iter().copied().enumerate() {
                        if child_weight != desired_weight {
                            let node_weight = tree.children[child_ind].weight;
                            let desired_weight = node_weight + desired_weight - child_weight;
                            return Err(desired_weight);
                        }
                    }

                    unreachable!()
                }
            }
        }
    }

    match dfs(&tree) {
        Ok(_) => {
            panic!("No unbalanced node detected");
        }
        Err(corrected) => corrected,
    }
}

pub fn run_7b() -> u32 {
    run_7b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_7a() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        let output = run_7a_with_input(input);

        assert_eq!(output, "tknk".to_string());
    }

    #[test]
    fn sample_7b() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        let output = run_7b_with_input(input);

        assert_eq!(output, 60);
    }
}
