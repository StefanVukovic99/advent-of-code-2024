use aoc_runner_derive::{aoc};

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules : Vec<(usize, usize)> = Vec::new();
    let mut examples : Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        match line {
            _ if line.contains("|") => {
                let parts = line.split("|").collect::<Vec<&str>>();
                rules.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
            },
            _ if line.contains(",") => {
                let parts = line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
                examples.push(parts);
            },
            _ => ()
        }
    }
    (rules, examples)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let (rules, examples) = parse_input(input);
    let mut sum = 0;

    'examples: for example in examples {
        for rule in rules.clone() {
            let pos1 = example.iter().position(|&x| x == rule.0);
            let pos2 = example.iter().position(|&x| x == rule.1);
            if pos1.is_some() && pos2.is_some() && pos1 > pos2 {
                continue 'examples;
            }
        }
        sum+= example[example.len()/2];
    }
    sum
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let (rules, examples) = parse_input(input);
    let mut sum = 0;

    for (i, example) in examples.iter().enumerate() {
        let mut sorted_example = example.clone();
        let mut had_to_sort = false;
        for rule in rules.clone() {
            let pos1 = example.iter().position(|&x| x == rule.0);
            let pos2 = example.iter().position(|&x| x == rule.1);
            if pos1.is_some() && pos2.is_some() && pos1 > pos2 {
                sorted_example.sort_by(|a, b| {
                    if *a == rule.0 && *b == rule.1 {
                        return std::cmp::Ordering::Less;
                    } else if *a == rule.1 && *b == rule.0 {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Equal;
                    }
                });
                had_to_sort = true;
            }
        }
        if had_to_sort {
            dbg!(example);
            sum += sorted_example[sorted_example.len()/2];
        }
    }
    sum
}