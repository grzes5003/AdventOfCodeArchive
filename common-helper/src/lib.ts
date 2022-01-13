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

export default parse;