use aoc_runner_derive::{aoc};

const MATRIX_SIZE: usize = 140;
const MATRIX_MAX: usize = MATRIX_SIZE - 1;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut matrix = [[' '; MATRIX_SIZE]; MATRIX_SIZE];
        
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            matrix[i][j] = c;
        });
    });

    fn search_vicinity(matrix: &[[char; 140]; 140], x: usize, y: usize) -> usize {
        let mut hits = 0;
        hits += search_direction(matrix, x, -1, y, -1); // up-left
        hits += search_direction(matrix, x, -1, y, 0); // up
        hits += search_direction(matrix, x, -1, y, 1); // up-right
        hits += search_direction(matrix, x, 0, y, -1); // left
        hits += search_direction(matrix, x, 0, y, 1); // right
        hits += search_direction(matrix, x, 1, y, -1); // down-left
        hits += search_direction(matrix, x, 1, y, 0); // down
        hits += search_direction(matrix, x, 1, y, 1); // down-right

        hits
    }

    fn search_direction(matrix: &[[char; 140]; 140], x: usize, dx: isize, y: usize, dy: isize) -> usize {
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
    let mut matrix = [[' '; MATRIX_SIZE]; MATRIX_SIZE];
        
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            matrix[i][j] = c;
        });
    });

    fn search_diagonal(matrix: &[[char; 140]; 140], x: usize, dx: isize, y: usize, dy: isize) -> bool {
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

    fn search_diagonals(matrix: &[[char; 140]; 140], x: usize, y: usize) -> usize {
        match (
            search_diagonal(matrix, x, 1, y, 1),
            search_diagonal(matrix, x, -1, y, 1),
            search_diagonal(matrix, x, 1, y, -1),
            search_diagonal(matrix, x, -1, y, -1),
        ) {
            (true, true, _, _) | 
            (true, _, true, _) |
            (_, true, _, true) |
            (_, _, true, true)
            => return 1,
            (_, _, _, _) => return 0,
        }
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