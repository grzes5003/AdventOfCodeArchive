interface Pattern {
    ab: [number, number],
    ch: string
    passwd: string
}

interface Pair {
    ab: [number, number],
    len: number
}

type Verify = (input: Pattern[]) => Promise<number>;

const task = (input: Pattern[], verify: Verify) => {
    return Promise.all(input.chunks(250).map(chunks =>
        verify(chunks)
    )).then((res: Array<number>) =>
        res.reduce((sum, current) => sum + current, 0)
    );
}

const verify01 = async (input: Pattern[]): Promise<number> => {
    return new Promise((resolve, _) =>
        resolve(input.map(pattern => {
                return {
                    ab: pattern.ab,
                    len: [...pattern.passwd].filter(ch => ch === pattern.ch).length
                } as Pair;
            }
        ).filter(({ab, len}: Pair) =>
            ab[0] <= len && ab[1] >= len
        ).length)
    )
}

const verify02 = async (input: Pattern[]): Promise<number> => {
    return new Promise((resolve, _) =>
        resolve(input.map(pattern => {
                const [a, b] = pattern.ab;
                return (pattern.passwd[a - 1] === pattern.ch) !=
                    (pattern.passwd[b - 1] === pattern.ch);
            }
        ).filter(val => val).length)
    )
}

export {
    task,
    verify01,
    verify02
}

export type {Pattern}