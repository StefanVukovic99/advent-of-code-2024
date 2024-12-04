use aoc_runner_derive::{aoc};

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let len_x = matrix.len();
    let len_y = matrix[0].len();

    fn search_vicinity(matrix: &Vec<Vec<char>>, x: usize, y: usize, len_x: usize, len_y: usize) -> usize {
        let mut hits = 0;
        for i in 0..9 {
            let dx = (i / 3) as isize - 1;
            let dy = (i % 3) as isize - 1;
            hits += search_direction(matrix, x, dx, y, dy, len_x, len_y);
        }
        hits
    }

    fn search_direction(matrix: &Vec<Vec<char>>, x: usize, dx: isize, y: usize, dy: isize, len_x: usize, len_y: usize) -> usize {
        let x_end = x as isize + 3 * dx;
        let y_end = y as isize + 3 * dy;

        if x_end < 0 || x_end >= len_x as isize || y_end < 0 || y_end >= len_y as isize {
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
    for i in 0..len_x {
        for j in 0..len_y {
            if matrix[i][j] == 'X' {
                hits += search_vicinity(&matrix, i, j, len_x, len_y);
            }
        }
    }
    
    hits
}

#[aoc(day4, part2)]
pub fn part2(_input: &str) -> u32 {
    2
}