import parse from './lib';

test('basic parse test', async () => {
    expect(await parse('../../aoc-2020/resources/day01_test.in')).toEqual(
        ["1721", "979", "366", "299", "675", "1456"]
    );
});