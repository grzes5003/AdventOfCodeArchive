
function ValueError(this: any, message = "") {
    this.message = message;
}
ValueError.prototype = Error.prototype;

interface Passport {
    byr: number,
    iyr: number,
    eyr: number,
    hgt: number,
    hcl: string,
    ecl: string,
    pid: string,
    cid?: string,
}

const parse_string = (input: string): Passport | Error => {
    const seq = new Map();
    input.split(' ').forEach(token => {
        const [a, b] = token.replace(' ', '')
            .split(':');
        seq.set(a,b);
    });

}

const visit_seq = (seq: Map<string,string>): Passport | Error => {
    const byr = Number(seq.get('byr')).between(1920,2003);
    const eyr = Number(seq.get('eyr')).between(2020,2031);
    const iyr = Number(seq.get('iyr')).between(2010,2021) ? Number(seq.get('iyr')) : throw new RangeError("as");
    const hgt;
    const hcl;
    const ecl;
    const pid;
    const cid;
    return {
        byr: byr,
        eyr: eyr,
        iyr: iyr,
        hgt: hgt,
        hcl: hcl,
        ecl: ecl,
        pid: pid,
        cid: cid
    }
}