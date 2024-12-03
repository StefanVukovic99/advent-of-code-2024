const fs = require('fs');

const input = fs.readFileSync('input-03.txt', 'utf-8');

const mulRegex = /mul\((\d+),(\d+)\)/g;
const doRegex = /do\(\)/g;
const dontRegex = /don't\(\)/g;  

const commands = input.split(doRegex).map(command => command.split(dontRegex)[0]);

console.log(commands);

let sumOfProducts = 0;
for (const command of commands) {
  const mulMatches = command.matchAll(mulRegex);

  sumOfProducts += [...mulMatches].reduce((acc, match) => acc + match[1] * match[2], 0);

  console.log(sumOfProducts);
}

