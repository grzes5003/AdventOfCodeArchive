
enum Arg {
    acc,
    jmp,
    nop
}

interface Token {
    line: number
    arg: Arg,
    val: number
}


const executor = (instr: Token[]) => {
    let seen = new Set<number>();
    let accumulator = 0;
    let idx = 0;
    while (1) {
        if (seen.has(idx) || instr.length == idx) return accumulator;
        seen.add(idx);
        const {line, arg, val} = instr[idx];
        // console.log(`${line}: ${Arg[arg]} ${val}`)
        switch (arg) {
            case Arg.acc: {
                accumulator += val;
                idx++;
                break;
            }
            case Arg.jmp: {
                idx += val;
                break;
            }
            default: {
                idx++;
                break;
            }
        }
    }
}


const parser = (input: string[]): Token[] =>
    input.map((value,idx) => {
        // @ts-ignore
        const [arg, val]: [string, string] = value.split(' ');
        return {
            line: idx,
            // @ts-ignore
            arg: Arg[arg],
            val: +val
        } as Token;
    })


const task01 = (input: string[]): number => {
    return executor(parser(input)) || -1;
}


export {
    task01
}