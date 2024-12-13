use aoc_runner_derive::{aoc};
use crate::utils::print_matrix;

const MATRIX_SIZE : usize = 50;
type Matrix = [[char; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
type Locations = HashMap<char, Vec<(usize, usize)>>;

use std::collections::HashMap;

fn parse_input(input: &str) -> Locations {
    let mut matrix = [['.'; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
    let mut locations : Locations = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[i][j] = c;
            if c != '.' {
                let entry = locations.entry(c).or_insert(Vec::new());
                entry.push((i, j));
            }
        }
    }
    print_matrix(&matrix.iter().map(|row| row.to_vec()).collect());
    locations
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let locations = parse_input(input);
    let mut blank_matrix = [['.'; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];

    for key in locations.keys() {
        let value = locations.get(key).unwrap();
        for i in 0..value.len() {
            for j in i+1..value.len() {
                let (x1, y1) = value[i];
                let (x2, y2) = value[j];
                let dx = x1 as i32 - x2 as i32;
                let dy = y1 as i32 - y2 as i32;
                let point1 = (x1 as i32 + dx, y1 as i32 + dy);
                let point2 = (x2 as i32 - dx, y2 as i32 - dy);
                mark_point(&mut blank_matrix, point1, '#');
                mark_point(&mut blank_matrix, point2, '#');
            }
        }
    }
    // println!("---");
    // print_matrix(&blank_matrix.iter().map(|row| row.to_vec()).collect());
    blank_matrix.iter().flatten().filter(|&&c| c == '#').count()
}

fn mark_point(matrix: &mut Matrix, point: (i32, i32), c: char) {
    let (x, y) = point;
    if x >= 0 && x < MATRIX_SIZE as i32 && y >= 0 && y < MATRIX_SIZE as i32 {
        matrix[x as usize][y as usize] = c;
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let locations = parse_input(input);
    let mut blank_matrix = [['.'; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];

    for key in locations.keys() {
        let value = locations.get(key).unwrap();
        for i in 0..value.len() {
            for j in i+1..value.len() {
                let (x1, y1) = (value[i].0 as i32, value[i].1 as i32);
                let (x2, y2) = (value[j].0 as i32, value[j].1 as i32);
                let dx = x1 - x2;
                let dy = y1 - y2;
                let mut k = 0;
                while (x1 + k*dx >= 0 && x1 + k*dx < MATRIX_SIZE as i32) && (y1 + k*dy >= 0 && y1 + k*dy < MATRIX_SIZE as i32) {
                    mark_point(&mut blank_matrix, (x1 + k*dx, y1 + k*dy), '#');
                    k += 1;
                }
                k = 0;
                while (x2 - k*dx >= 0 && x2 - k*dx < MATRIX_SIZE as i32) && (y2 - k*dy >= 0 && y2 - k*dy < MATRIX_SIZE as i32) {
                    mark_point(&mut blank_matrix, (x2 - k*dx, y2 - k*dy), '#');
                    k += 1;
                }
            }
        }
    }
    println!("---");
    print_matrix(&blank_matrix.iter().map(|row| row.to_vec()).collect());
    blank_matrix.iter().flatten().filter(|&&c| c == '#').count()
}
