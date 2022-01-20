const countAllAnswers = (input: string): number => {
    const ans = new Set<string>();
    input.replace(/[\W_]+/g, '')
        .split('')
        .forEach(char => ans.add(char));
    return ans.size;
}

const countCommonAnswers = (input: string[]): number => {
    let common = input[0].split('');
    input.filter(value => value.length !== 0)
        .forEach(value => {
            common = value
                .split('')
                .filter(val => common.includes(val));
        })
    return common.length;
}

const task01 = (input: string[]): number => {
    let parsed_input = [];
    let tmp = '';
    for (let line of input) {
        tmp += line;
        if (line === '') {
            parsed_input.push(tmp);
            tmp = '';
        }
    }
    parsed_input.push(tmp);
    return parsed_input
        .map(token => countAllAnswers(token))
        .reduce((acc, curr) => acc + curr, 0);
}


const task02 = (input: string[]): number => {
    let parsed_input = [];
    let tmp = [];
    for (let line of input) {
        tmp.push(line);
        if (line.length === 0) {
            parsed_input.push(tmp);
            tmp = [];
        }
    }
    parsed_input.push(tmp);
    return parsed_input
        .map(value => countCommonAnswers(value))
        .reduce((previousValue, currentValue) => previousValue + currentValue, 0);
}

export {
    countAllAnswers,
    task01,
    task02
}