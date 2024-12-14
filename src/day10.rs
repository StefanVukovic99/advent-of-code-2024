use aoc_runner_derive::{aoc};

const MATRIX_SIZE: usize = 50;
const DIRECTIONS : [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_input(input: &str) -> [[u8; MATRIX_SIZE]; MATRIX_SIZE] {
    let mut matrix = [[0; MATRIX_SIZE]; MATRIX_SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[i][j] = c.to_digit(10).unwrap() as u8;
        }
    }
    matrix

}

fn get_reachable_nines_recursive(matrix: &[[u8; MATRIX_SIZE]; MATRIX_SIZE], x: usize, y: usize) -> Vec<(usize, usize)> {
    let number = matrix[x][y];
    if(number == 9) {
        return vec![(x, y)];
    }

    let mut empty : Vec<(usize, usize)> = vec![];
    let value =  DIRECTIONS.iter().fold(empty, |mut acc, (dx, dy)| {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x < 0 || new_x >= MATRIX_SIZE as isize || new_y < 0 || new_y >= MATRIX_SIZE as isize {
            return acc;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        if matrix[new_x][new_y] == number + 1 {
            acc.append(&mut get_reachable_nines_recursive(matrix, new_x, new_y));
        }
        return acc;
    });

    let value = value.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>();

    // println!("Value of {} at ({}, {}) is {:?}", number, x, y, value);

    value
    
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut matrix = [[0; MATRIX_SIZE]; MATRIX_SIZE];
    let mut zeroes : Vec<(usize, usize)> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[i][j] = c.to_digit(10).unwrap() as u8;
            if matrix[i][j] == 0 {
                zeroes.push((i, j));
                
            }
        }
    }
    
    // return sum of values of zeroes
    zeroes.iter().fold(0, |acc, (x, y)| {
        acc + get_reachable_nines_recursive(&matrix, *x, *y).len()
    })
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let matrix = parse_input(input);
    0
    
}