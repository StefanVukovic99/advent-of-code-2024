const fs = require('fs');

const input = fs.readFileSync('input-04.txt', 'utf-8');

const matrix = input.split('\n').map(line => line.split(''));
const LEN_X = matrix.length;
const LEN_Y = matrix[0].length;

function searchDiagonals(x, y) {
    const diagonals = [
        [1, 1],
        [1, -1],
        [-1, -1],
        [-1, 1]
    ]
    for (let i = 0; i<4; i+=1){
        const [dx1, dy1] = diagonals[i];
        const [dx2, dy2] = diagonals[(i+1)%4];
        if(searchDiagonal(x, dx1, y, dy1) && searchDiagonal(x, dx2, y, dy2)) return 1;
    }
    return 0;
}



function searchDiagonal(x, dx, y, dy) {

    if(x == 0 || x == LEN_X - 1) return 0;
    if(y == 0 || y == LEN_Y - 1) return 0;
    if(matrix[x+dx][y+dy] != 'M') return 0;
    if(matrix[x-dx][y-dy] != 'S') return 0;
    return 1;
}

let hits = 0;
for (let i = 0; i < LEN_X; i++) {
    for (let j = 0; j < LEN_Y; j++) {
        if (matrix[i][j] === 'A') {
            hits += searchDiagonals(i, j);
        }
    }
}

console.log(hits);  