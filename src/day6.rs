use aoc_runner_derive::{aoc};
use std::fs::File;
use std::io::{Write, BufWriter};

type Matrix = [[char; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
type NumberMatrix = [[usize; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
type Pos = (isize, isize);

const MATRIX_SIZE: isize = 130;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const ARROWS: [char; 4] = ['^', '>', 'v', '<'];
const PRIMES: [usize; 4] = [2, 3, 5, 7];

fn get_prime_factors(n: usize) -> Vec<usize> {
    if n < 4 {
        return vec![n];
    }
    let mut n = n;
    let mut factors = Vec::new();
    for &prime in PRIMES.iter() {
        while n % prime == 0 {
            factors.push(prime);
            n /= prime;
        }
    }
    factors
}



fn get_arrow_for_prime(prime: usize) -> char {
    match prime {
        0 => '#',
        1 => '.',
        2 => '^',
        3 => '>',
        5 => 'v',
        7 => '<',
        6 => '+',
        14 => '+',
        15 => '+',
        35 => '+',
        _ => '?',
    }
    
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for &cell in row {
            print!("{} ", cell);
        }
        println!(); // New line after each row
    }
}

fn save_matrix_to_file(matrix: &Vec<Vec<char>>, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for row in matrix {
        let row_string: String = row.iter()
            .map(|&c| format!("{} ", c))
            .collect();
        writeln!(writer, "{}", row_string.trim())?;
    }

    Ok(())
}

// return matrix of chars
fn parse_input(input: &str) -> (Matrix, Pos) {
    let mut matrix = [['.'; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
    let mut guard_pos = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[i][j] = c;
            if c == '^' {
                guard_pos = (i as isize, j as isize);
            }
        }
    }
    (matrix, guard_pos)
}

fn get_number_matrix(matrix: &Matrix) -> NumberMatrix {
    let mut number_matrix = [[1; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            number_matrix[i as usize][j as usize] = match matrix[i as usize][j as usize] {
                '#' => 0,
                _ => 1,
            };
        }
    }
    number_matrix
}

fn convert_number_matrix(matrix: &NumberMatrix) -> Matrix {
    let mut new_matrix = [['.'; MATRIX_SIZE as usize]; MATRIX_SIZE as usize];
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            new_matrix[i as usize][j as usize] = get_arrow_for_prime(matrix[i as usize][j as usize]);
        }
    }
    new_matrix
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (mut matrix, mut guard_pos) = parse_input(input);
    let mut direction_index = 0;

    loop {
        matrix[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
        let direction = DIRECTIONS[direction_index];
        let next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        if next_pos.0 < 0 || next_pos.0 >= MATRIX_SIZE || next_pos.1 < 0 || next_pos.1 >= MATRIX_SIZE {
            break;
        }
        if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction_index = (direction_index + 1) % 4;
            continue;
        }
        guard_pos = (next_pos.0, next_pos.1);
    }

    matrix.iter().map(|row| row.iter().filter(|&&c| c == 'X').count()).sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (og_matrix, og_guard_pos) = parse_input(input);
    let mut direction_index = 0;
    // number matrix is all 1s
    let mut matrix = og_matrix.clone();
    let mut guard_pos = og_guard_pos;
    let mut number_matrix = get_number_matrix(&matrix);

    // save_matrix_to_file(&matrix.iter().map(|row| row.to_vec()).collect(), "output-06-1-01.txt").unwrap();

    let mut i = 0;
    loop {
        i += 1;

        matrix[guard_pos.0 as usize][guard_pos.1 as usize] = ARROWS[direction_index];
        number_matrix[guard_pos.0 as usize][guard_pos.1 as usize] *= PRIMES[direction_index] as usize;

        // let converted_matrix = convert_number_matrix(&number_matrix);
        // save_matrix_to_file(&converted_matrix.iter().map(|row| row.to_vec()).collect(), format!("output-06-1-{i}.txt").as_str()).unwrap();
    
        let direction = DIRECTIONS[direction_index];
        let next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        if next_pos.0 < 0 || next_pos.0 >= MATRIX_SIZE || next_pos.1 < 0 || next_pos.1 >= MATRIX_SIZE {
            break;
        }
        if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction_index = (direction_index + 1) % 4;
            continue;
        }
        guard_pos = (next_pos.0, next_pos.1);
    }

    let converted_matrix = convert_number_matrix(&number_matrix);
    // save_matrix_to_file(&converted_matrix.iter().map(|row| row.to_vec()).collect(), "output-06-1-02.txt").unwrap();

    let mut blockades: Vec<(usize, usize)> = Vec::new();

    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            if number_matrix[i as usize][j as usize] > 1 {
                let mut matrix = og_matrix.clone();
                matrix[i as usize][j as usize] = '#';
                let mut number_matrix = get_number_matrix(&matrix);
                // let converted_matrix = convert_number_matrix(&number_matrix);
                // save_matrix_to_file(&converted_matrix.iter().map(|row| row.to_vec()).collect(), format!("output-06-2-blocked-{i}-{j}-0.txt").as_str()).unwrap();
                print!("Trying to replace ({i}, {j})\n");

                let mut guard_pos = og_guard_pos;
                let mut direction_index = 0;
                let mut step = 0;
                'steps: loop {   
                    step += 1;         
                    matrix[guard_pos.0 as usize][guard_pos.1 as usize] = ARROWS[direction_index];
                    number_matrix[guard_pos.0 as usize][guard_pos.1 as usize] *= PRIMES[direction_index] as usize;
                    let prime_factors = get_prime_factors(number_matrix[guard_pos.0 as usize][guard_pos.1 as usize]);
                    //if same prime factor is repeated thrice, then we have a loop
                    if prime_factors.len() > 2 {
                        for prime in PRIMES {
                            if prime_factors.iter().filter(|&&p| p == prime).count() >= 3 {
                                // dbg!("Found loop");
                                blockades.push((i as usize,j as usize));
                                // let converted_matrix = convert_number_matrix(&number_matrix);
                                // save_matrix_to_file(&converted_matrix.iter().map(|row| row.to_vec()).collect(), format!("output-06-2-{i}-{j}.txt").as_str()).unwrap();
                                break 'steps;
                            }
                        }                        
                   }
                    // let converted_matrix = convert_number_matrix(&number_matrix);
                    // save_matrix_to_file(&converted_matrix.iter().map(|row| row.to_vec()).collect(), format!("output-06-2-blocked-{i}-{j}-{step}.txt").as_str()).unwrap();
                
                    let direction = DIRECTIONS[direction_index];
                    let next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
                    if next_pos.0 < 0 || next_pos.0 >= MATRIX_SIZE || next_pos.1 < 0 || next_pos.1 >= MATRIX_SIZE {
                        // print!("breaking on step {step} because out of bounds\n");
                        break;
                    }
                    if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                        direction_index = (direction_index + 1) % 4;
                        continue;
                    }
                    guard_pos = (next_pos.0, next_pos.1);
                }
            }
        }
    }
    // dbg!(&blockades);
    blockades.len() 
}
