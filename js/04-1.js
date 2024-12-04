const fs = require('fs');

const input = fs.readFileSync('input-04.txt', 'utf-8');

const matrix = input.split('\n').map(line => line.split(''));
const LEN_X = matrix.length;
const LEN_Y = matrix[0].length;

function searchVicinity(x, y) {
    let hits = 0;
    for(let i = 0; i < 9; i++){
        dx = Math.floor(i / 3) - 1;
        dy = i % 3 - 1;
        hits += searchDirection(x, dx, y, dy)
    }
    return hits;
}

function searchDirection(x, dx, y, dy) {
    const x_end = x + 3*dx;
    const y_end = y + 3*dy;
    console.log('Searching direction', x, y, dx, dy, x_end, y_end);

    if(x_end < 0 || x_end > LEN_X - 1) return 0;
    if(y_end < 0 || y_end > LEN_Y - 1) return 0;
    if(matrix[x+dx][y+dy] != 'M') return 0;
    if(matrix[x+2*dx][y+2*dy] != 'A') return 0;
    if(matrix[x+3*dx][y+3*dy] != 'S') return 0;
    return 1;
}

let hits = 0;
for (let i = 0; i < LEN_X; i++) {
    for (let j = 0; j < LEN_Y; j++) {
        if (matrix[i][j] === 'X') {
            hits += searchVicinity(i, j);
        }
    }
}

console.log(hits);