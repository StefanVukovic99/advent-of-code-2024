const fs = require('fs')

const input = fs.readFileSync('input-05.txt', 'utf-8')

const [rawRules, rawExamples] = input.split('\n\n');

const rules = rawRules.split('\n').map(raw => raw.split('|').map(Number))
const examples = rawExamples.split('\n').map(raw => raw.split(',').map(Number))


let sum = examples
    .filter(example => {
        // console.log('testing example', example)
        return !rules.every(rule => {
            // console.log('\t testing rule', rule)
            const pos1 = example.findIndex(num => num == rule[0])
            const pos2 = example.findIndex(num => num == rule[1])
            if(pos1 == -1 || pos2 == -1 || pos1 < pos2) return true;
            // console.log('\t return false');
            return false;
        })
    })
    .map(fixBadExample)
    .reduce((acc, curr) => {
        console.log(curr)
        return acc + curr[(curr.length - 1) / 2]
    }, 0)


function fixBadExample(badExample){
    return badExample.toSorted((a, b) => {
        if(rules.find(([ruleA, ruleB]) => ruleA == a && ruleB == b)) return -1;
        if(rules.find(([ruleA, ruleB]) => ruleA == b && ruleB == a)) return 1;
        return NaN;
    })
}

console.log(sum)