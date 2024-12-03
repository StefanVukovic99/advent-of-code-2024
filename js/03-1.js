const fs = require('fs');

const input = fs.readFileSync('input-03.txt', 'utf-8');

const mulRegex = /mul\((\d+),(\d+)\)/g;

const mulMatches = input.matchAll(mulRegex);

const sumOfProducts = [...mulMatches].reduce((acc, match) => acc + match[1] * match[2], 0);

console.log(sumOfProducts);