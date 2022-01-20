import parse from "../../common-helper/src/lib";
import {task01, task02} from "./day05";

let arr: string[];

beforeAll(async () => {
    arr = await parse('../../aoc-2020/resources/day05.in');
})

test('day05_1', async () => {
    console.log(await task01(arr))
})

test('day05_2', async () => {
    console.log(await task02(arr))
})