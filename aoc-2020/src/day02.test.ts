import parse from "../../common-helper/src/lib";
import {task,verify01,verify02,Pattern} from "./day02";

let arr: Pattern[];

const read = (input: string[]): Pattern[] => {
    return input.map(line => {
        const [ab, ch, passwd] = line.split(' ');
        return {
            // @ts-ignore
            ab: [...ab.split('-').map(Number)],
            ch: ch.replace(':', ''),
            passwd
        } as Pattern;
    })
}

beforeAll(async () => {
    arr = await parse('../../aoc-2020/resources/day02.in')
        .then(res => read(res));
})

test('day02_01', async () => {
    console.log(await task(arr, verify01));
})

test('day02_02', async () => {
    console.log(await task(arr, verify02));
})