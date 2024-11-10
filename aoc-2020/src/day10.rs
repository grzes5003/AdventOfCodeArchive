use std::collections::HashMap;

type Num = u32;

fn task01(input: &Vec<Num>) -> Num {
    let mut inp = input.clone();
    inp.sort();
    let res: Vec<_> = inp
        .windows(2)
        .map(|slice| {
            if let [l, r] = slice {
                r - l
            } else {
                panic!("Wont happened");
            }
        })
        .collect();
    let ones = res.iter().filter(|&&x| x == 1).count() as Num + 1;
    let threes = res.iter().filter(|&&x| x == 3).count() as Num + 1;
    ones * threes
}

fn perm(input: &[Num], cache: &mut HashMap<Num, u64>) -> u64 {
    match input {
        [a, _, c, d, ..] => {
            if let Some(val) = cache.get(a) {
                *val
            } else {
                let result = (if c - a <= 3 {
                    perm(&input[2..], cache)
                } else {
                    0
                }) + (if d - a <= 3 {
                    perm(&input[3..], cache)
                } else {
                    0
                }) + perm(&input[1..], cache);
                cache.insert(*a, result);
                result
            }
        }
        [a, _, c] => {
            if c - a <= 3 {
                2
            } else {
                1
            }
        }
        [_, _] | [_] | [] => 1,
    }
}

fn task02(input: &Vec<Num>) -> u64 {
    let mut inp = input.clone();
    inp.sort();
    inp.insert(0, 0);
    inp.push(inp.last().unwrap() + 3);
    let mut cache = HashMap::new();
    perm(&inp[..], &mut cache)
}

#[cfg(test)]
mod tests {
    use super::{task01, task02, Num};
    use common_helper::parse;

    fn input_data(path: String) -> Vec<Num> {
        parse(&path)
            .iter()
            .map(|num| num.parse().unwrap())
            .collect()
    }

    #[test]
    fn task01_test() {
        let vec = input_data("resources/day10_test.in".to_string());
        let result = task01(&vec);
        println!("task01: {:?}", result)
    }

    #[test]
    fn task01_task() {
        let vec = input_data("resources/day10.in".to_string());
        let result = task01(&vec);
        println!("task01: {:?}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data("resources/day10_test.in".to_string());
        let result = task02(&vec);
        println!("task02: {:?}", result)
    }

    #[test]
    fn task02_task() {
        let vec = input_data("resources/day10.in".to_string());
        let result = task02(&vec);
        println!("task02: {:?}", result)
    }
}
