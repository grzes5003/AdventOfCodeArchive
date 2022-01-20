import {once} from 'events';
import {createReadStream} from 'fs';
import path from 'path';
import * as readline from 'readline';


const parse = async (_path: string) => {
    const pt = path.join(__dirname, _path);
    // console.log(`started ${pt}`);
    const input = createReadStream(pt, 'utf8');
    const rl = readline.createInterface({
        input,
        crlfDelay: Infinity
    });

    const lines: string[] = [];
    rl.on('line', (line) => lines.push(line));

    await once(rl, 'close');
    return lines;
}

interface IteratorResult<T> {
    done: boolean;
    value: T;
}

type Iterator<T> = {
    next(value?: any): IteratorResult<T>;
    return?(value?: any): IteratorResult<T>;
    throw?(e?: any): IteratorResult<T>;
}

declare global {
    export interface Array<T> {
        chunks: (size: number) => Array<Array<T>>;
    }
}

Array.prototype.chunks = function(size: number) {
    const arrays = [];
    while (this.length > 0) {
        arrays.push(this.splice(0, size));
    }
    return arrays;
}

declare global {
    export interface Number {
        between: (a: number, b: number) => boolean;
    }
}

Number.prototype.between = function(a, b) {
    const   min = Math.min.apply(Math, [a, b]),
            max = Math.max.apply(Math, [a, b]);
    return this >= min && this < max;
};

export default parse;