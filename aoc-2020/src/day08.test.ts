import parse from "../../common-helper/src/lib";
import {task01} from "./day08";

let arr: string[];

beforeAll(async () => {
    arr = await parse('../../aoc-2020/resources/day08.in');
})

test('day07_1', () => {
    console.log(task01(arr))
})
