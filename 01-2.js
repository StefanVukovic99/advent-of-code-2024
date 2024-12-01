const fs = require('fs');

fs.readFile('input-01-1.csv', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    const list1 = [];
    const list2 = [];

    data.split('\n').forEach((line, index) => {
        const parts = line.split(/\s+/);
        if(parts.length !== 2) {
            console.error('Invalid line:', index + 1);
            return;
        }
        const [a, b] = parts;
        list1.push(parseInt(a));
        list2.push(parseInt(b));
    });

    list1.sort((a, b) => a - b);
    list2.sort((a, b) => a - b);

    console.log(list1);
    console.log(list2);

    const similarityScore = list1.reduce((acc, value, index) => {
        // count occurences of value in list2
        const count = list2.filter((v) => v === value).length;
        return acc + count * value;
    }, 0);

    console.log('Similarity score:', similarityScore);
});