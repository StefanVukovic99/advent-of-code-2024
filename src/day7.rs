use aoc_runner_derive::{aoc};

type Equation = (usize, Vec<usize>);

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let parts = line.split_once(':').unwrap();
        let target = parts.0.parse::<usize>().unwrap();
        let mut results : Vec<usize> = Vec::new();
        let operands = parts.1.trim().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        
        results.push(operands[0]);
        
        for i in 1..operands.len() {
            for j in (0..results.len()).rev() {
                let added = results[j] + operands[i];
                let multiplied = results[j] * operands[i];
                let append = [added, multiplied];
                
                results.splice(j..j+1, append);
            }

        }

        if results.contains(&target) {
            sum += target;
        }

    }
    sum
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let parts = line.split_once(':').unwrap();
        let target = parts.0.parse::<usize>().unwrap();
        let mut results : Vec<usize> = Vec::new();
        let operands = parts.1.trim().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        
        results.push(operands[0]);
        
        for i in 1..operands.len() {
            for j in (0..results.len()).rev() {
                let added = results[j] + operands[i];
                let multiplied = results[j] * operands[i];
                let concatenated = (results[j].to_string() + &operands[i].to_string()).parse::<usize>().unwrap();
                let append = [added, multiplied, concatenated].iter().filter(|&x| *x <= target).map(|x| *x).collect::<Vec<usize>>();
                
                results.splice(j..j+1, append);
            }

        }

        if results.contains(&target) {
            sum += target;
        }

    }
    sum

}
