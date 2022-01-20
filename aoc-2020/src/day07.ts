interface BagItem {
    name: string,
    num: number
}
const instanceOfBagItem = (object: any): object is BagItem => 'num' in object;

interface Bag {
    name: string,
    inside: BagItem[] | [Empty]
}

interface Empty {}
const instanceOfEmpty = (object: any): object is Empty =>
    Object.keys(object).length === 0;

interface Total {
    color: Set<string>
    num: number[]
}

// "" -> { 2: "", 4: "" }


const NO_OTHER_BAGS = 'no other bags.';


const parseBag = (token: string): BagItem | Empty => {
    if (token === NO_OTHER_BAGS) return {} as Empty;
    const split = token.split(' ').filter(val => val.length !== 0);
    return {
        name: split.slice(1, 3).join(' '),
        num: +split[0]
    } as BagItem;
}


const parser = (input: string[]): Bag[] => {
    return input.map(line => {
        const words = line.split(' ');
        const bagName = words.slice(0, 2).join(' ');
        const inside = words.slice(4).join(' ')
            .split(',')
            .map(token => parseBag(token));
        return {
            name: bagName,
            inside
        } as Bag
    });
}

// @ts-ignore
const countBags = (name: string, bags: Bag[], total: Total): number => {
    const bag = bags.find(o => o.name === name);
    console.log(`===> ${name}`)
    if (instanceOfEmpty(bag?.inside[0])) return 1;
    const eff = bag?.inside
            .map(bag => {
                // @ts-ignore
                const a = countBags(bag.name, bags, total);
                    // @ts-ignore
                console.log(bag.num, ' * ', a);
                    // @ts-ignore
                total.num.push(bag.num * a);
                // @ts-ignore
                return bag.num * a;
                }
            );
    return eff?.slice(-1)[0] || -1;
}

const resolver = (name: string, bags: Bag[], total: Total): Total => {
    bags.filter(bag => bag.inside?.find(val => instanceOfBagItem(val) && val.name === name))
        .forEach(bag => {
            total.color.add(bag.name);
            resolver(bag.name, bags, total);
        });
    return total
}

const task01 = (input: string[]): number => {
    const bags: Bag[] = parser(input);
    const res = resolver('shiny gold', bags, {color: new Set(), num: [0]});
    return res.color.size;
}

const task02 = (input: string[], name: string): number => {
    const bags: Bag[] = parser(input);
    const total = {num: [0]} as Total;
    const res = countBags(name, bags, total);
    console.log(total)
    return total.num.reduce((previousValue, currentValue) => previousValue + currentValue, 0);
}

export {
    task01,
    task02
}