import parse from './lib';

test('adds 1 + 2 to equal 3', async () => {
    expect(await parse('../../../resources/day01_test.in'));
});