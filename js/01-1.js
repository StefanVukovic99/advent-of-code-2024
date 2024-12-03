const fs = require('fs');

fs.readFile('input-01.txt', 'utf8', (err, data) => {
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

    //sort lists in place
    list1.sort((a, b) => a - b);
    list2.sort((a, b) => a - b);

    // Use the input data here
    console.log(list1);
    console.log(list2);

    const difference = list1.reduce((acc, value, index) => {
        return acc + Math.abs(value - list2[index]);
    }
    , 0);

    console.log('Difference:', difference);
});