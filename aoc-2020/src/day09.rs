use std::collections::{HashSet, VecDeque};

type Num = u64;

fn has_pair<'a>(items: &'a [Num], target: Num) -> bool {
    let mut hs = HashSet::<Num>::new();
    for item in items {
        if hs.contains(&((item).abs_diff(target))) {
            return true;
        }
        hs.insert(*item);
    }
    false
}

fn task01(input: &Vec<Num>) -> Option<Num> {
    let window = input.windows(26);
    for item in window {
        let (arr, [target]) = item.split_last_chunk::<1>().unwrap();
        if !has_pair(arr, *target) {
            return Some(*target);
        }
    }
    None
}

fn task02(input: &[Num], target: Num) -> Option<Num> {
    let mut sum = 0;
    let mut buff = VecDeque::new();
    let mut arr = input.into_iter();
    loop {
        if sum == target {
            return buff
                .iter()
                .max()
                .and_then(|max| buff.iter().min().map(|min| *min + *max));
        }

        if sum < target {
            if let Some(s) = arr.next() {
                buff.push_back(s);
                sum += s;
            } else {
                return Option::None;
            }
        } else {
            let el = buff.pop_front().unwrap();
            sum -= el;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{task01, task02, Num};
    use common_helper::parse;

    fn input_data() -> Vec<Num> {
        parse("resources/day09.in")
            .iter()
            .map(|num| num.parse().unwrap())
            .collect()
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(&vec);
        println!("task01: {:?}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let target = task01(&vec).unwrap();
        let result = task02(&vec, target);
        println!("task01: {:?}", result)
    }
}
