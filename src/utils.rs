pub fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for &cell in row {
            print!("{} ", cell);
        }
        println!(); // New line after each row
    }
}
