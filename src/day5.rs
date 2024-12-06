use aoc_runner_derive::{aoc};

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules = Vec::new();
    let mut examples = Vec::new();
    
    for line in input.lines() {
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            rules.push((
                parts[0].parse().unwrap(), 
                parts[1].parse().unwrap()
            ));
        } else if line.contains(',') {
            let parts: Vec<usize> = line.split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            examples.push(parts);
        }
    }
    
    (rules, examples)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let (rules, examples) = parse_input(input);
    
    examples.iter()
        .filter(|&example| {
            rules.iter().all(|&rule| {
                let pos1 = example.iter().position(|&x| x == rule.0);
                let pos2 = example.iter().position(|&x| x == rule.1);
                
                !(pos1.is_some() && pos2.is_some() && pos1 > pos2)
            })
        })
        .map(|example| example[example.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let (rules, examples) = parse_input(input);
    
    examples.iter()
        .filter_map(|example| {
            // Check if sorting is needed
            let needs_sorting = rules.iter().any(|&rule| {
                let pos1 = example.iter().position(|&x| x == rule.0);
                let pos2 = example.iter().position(|&x| x == rule.1);
                pos1.is_some() && pos2.is_some() && pos1 > pos2
            });
            
            if needs_sorting {
                let mut sorted_example = example.clone();
                sorted_example.sort_by(|&a, &b| {
                    rules.iter()
                        .find_map(|&rule| {
                            if a == rule.0 && b == rule.1 {
                                Some(std::cmp::Ordering::Greater)
                            } else if a == rule.1 && b == rule.0 {
                                Some(std::cmp::Ordering::Less)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                
                Some(sorted_example[sorted_example.len() / 2])
            } else {
                None
            }
        })
        .sum()
}