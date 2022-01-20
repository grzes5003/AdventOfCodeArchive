import parse from "../../common-helper/src/lib";
import {task01, task02} from "./day07";

let arr: string[];

beforeAll(async () => {
    arr = await parse('../../aoc-2020/resources/day07_test.in');
})

test('day07_1', () => {
    console.log(task01(arr))
})


test('day07_2', () => {
    console.log(task02(arr,'shiny gold'))
    console.log(task02(arr,'dark olive'))
    console.log(task02(arr,'vibrant plum'))
})