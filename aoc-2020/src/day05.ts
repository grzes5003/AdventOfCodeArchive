interface Limit {
    a: number,
    b: number
}

interface direction {
    dir: { [_: string]: number },
    limit: Limit,
    len: number
}

const Row = {
    dir: {'F': 0, 'B': 1},
    limit: {a: 0, b: 127},
    len: 7,
} as direction;

const Cols = {
    dir: {'L': 0, 'R': 1},
    limit: {a: 0, b: 7},
    len: 3,
} as direction;

const middle = (a: number, b: number): number => Math.floor((b - a) / 2 + a)

const find_seat = async ([input, tp]: [string, direction]): Promise<number> => {
    let limit = tp.limit;
    input.split('').forEach(char => {
        const mid = middle(limit.a, limit.b);
        const arr = [{a: limit.a, b: mid}, {a: mid, b: limit.b}] as Limit[];
        limit = arr[tp.dir[char]]
    })
    return limit.b;
}

const get_IDs = async (input: string[]): Promise<number[]> =>
    Promise.all(input.map(async line =>
        [await find_seat([line.substring(0, Row.len), Row]), await find_seat([line.substring(Row.len), Cols])]
    ).map((x) => x.then((a) => 8 * a[0] + a[1])))



export const task01 = async (input: string[]): Promise<number> => {
    return Math.max(...await get_IDs(input));
}

export const task02 = async (input: string[]): Promise<number> => {
    const res = await get_IDs(input).then((x) => x.sort());
    const missing: number[] = [];
    [...Array(Math.max(...res)).keys()].forEach(val => {
        if (res.indexOf(val) == -1) {
            missing.push(val);
        }
        if (missing.indexOf(val - 2) != -1 && missing.indexOf(val - 1) == -1 && missing.indexOf(val) != -1) {
            console.log(val)
            return val - 1;
        }
    });
    return 1;
}