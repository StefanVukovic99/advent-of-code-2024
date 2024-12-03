const fs = require('fs');

function isReportSafe(report) {
    if (!Array.isArray(report)) {
        console.log('Invalid report:', report);
        return false;
    }
    if (report.length < 2) {
        console.log('Trivial report:', report);
        return false; // apparently, this is not a safe report
    }
    for (let i = 0; i < report.length - 1; i++) {
        const diff = report[i + 1] - report[i];
        if (Math.abs(diff) > 3 || Math.abs(diff) < 1) {
            return false;
        }
        if (Math.sign(diff) !== Math.sign(report[1] - report[0])) {
            return false;
        }
    }
    return true;
};

const reports = fs.readFileSync('input-02.txt', 'utf8')
    .split('\n')
    .map(line => line.split(/\s+/).map(Number));

const safe = reports.reduce((acc, report) => {
    const subsets = report.map((_, i) => {
        const copy = [...report];
        copy.splice(i, 1)
        return copy;
    });
    if(subsets.some(isReportSafe)) {
        return acc + 1;
    }
    return acc;
}, 0);

console.log(safe);
