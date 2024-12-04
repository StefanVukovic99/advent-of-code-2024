use aoc_runner_derive::{aoc};

const MATRIX_SIZE: usize = 140;
const MATRIX_MAX: usize = MATRIX_SIZE - 1;
const DIRECTIONS_ITER: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const DIAGONALS_ITER: [(isize, isize); 4] = [
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    fn search_vicinity(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
        let mut hits = 0;
        for (dx, dy) in DIRECTIONS_ITER {
            hits += search_direction(matrix, x, dx, y, dy);
        }
        hits
    }

    fn search_direction(matrix: &Vec<Vec<char>>, x: usize, dx: isize, y: usize, dy: isize) -> usize {
        let x_end = x as isize + 3 * dx;
        let y_end = y as isize + 3 * dy;

        if x_end < 0 || x_end > MATRIX_MAX as isize || y_end < 0 || y_end > MATRIX_MAX as isize {
            return 0;
        }

        let (x1, y1) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        let (x2, y2) = ((x as isize + 2 * dx) as usize, (y as isize + 2 * dy) as usize);
        let (x3, y3) = ((x as isize + 3 * dx) as usize, (y as isize + 3 * dy) as usize);

        if matrix[x1][y1] != 'M' || matrix[x2][y2] != 'A' || matrix[x3][y3] != 'S' {
            return 0;
        }
        return 1 as usize;
    }

    let mut hits = 0;
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            if matrix[i][j] == 'X' {
                hits += search_vicinity(&matrix, i, j);
            }
        }
    }
    
    hits
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    fn search_diagonal(matrix: &Vec<Vec<char>>, x: usize, dx: isize, y: usize, dy: isize) -> bool {
        let x_pos = (x as isize + dx) as usize;
        let y_pos = (y as isize + dy) as usize;

        let x_neg = (x as isize - dx) as usize;
        let y_neg = (y as isize - dy) as usize;

        if x == 0 || x == MATRIX_MAX || y == 0 || y == MATRIX_MAX {
            return false;
        }
        if matrix[x_pos][y_pos] != 'M' || matrix[x_neg][y_neg] != 'S' {
            return false;
        }

        true
    }

    fn search_diagonals(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
        for i in 0..4 {
            let (dx1, dy1) = DIAGONALS_ITER[i];
            let (dx2, dy2) = DIAGONALS_ITER[(i + 3) % 4];
            if search_diagonal(matrix, x, dx1, y, dy1) 
                && search_diagonal(matrix, x, dx2, y, dy2) {
                return 1;
            }
        }

        0
    }

    let mut hits = 0;
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE{
            if matrix[i][j] == 'A' {
                hits += search_diagonals(&matrix, i, j);
            }
        }
    }

    hits
}