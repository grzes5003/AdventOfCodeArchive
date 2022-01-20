import parse from "../../common-helper/src/lib";
import {task01,task02} from "./day06";

let arr: string[];

beforeAll(async () => {
    arr = await parse('../../aoc-2020/resources/day06.in');
})

test('day06_1', () => {
    console.log(task01(arr))
})

test('day06_2', () => {
    console.log(task02(arr))
})