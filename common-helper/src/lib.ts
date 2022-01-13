import { once } from 'events';
import { createReadStream } from 'fs';
import path from 'path';
import * as readline from 'readline';


const parse = async (_path: string) => {
    console.log('started');
    const pt = path.join(__dirname, _path);
    console.log(`started ${pt}`);
    const input = createReadStream(pt, 'utf8');
    // console.log(input)
    const rl = readline.createInterface({
        input,
        crlfDelay: Infinity
    });

    const lines: string[] = [];
    rl.on('line', (line) => lines.push(line));

    await once(rl, 'close');
    return lines;
}

export default parse;