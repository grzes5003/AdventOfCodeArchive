use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::iter;
use std::iter::Repeat;
use std::rc::Rc;

const TARGET: u32 = 2020;

fn task01(input: Vec<u32>) -> u32 {
    let mut seen = HashSet::<u32>::new();
    for num in input {
        if seen.contains(&(TARGET - num)) {
            return (TARGET - num) * num;
        }
        seen.insert(num);
    }
    panic!("No answer found")
}

fn task02(mut input: Vec<u32>) -> u32 {
    let input = input.iter()
        .flat_map(|&x| iter::repeat(x).zip(input.clone()).map(|(_1,_2)| (_1+_2,_2)))
        .collect::<HashMap<u32,u32>>();
    let mut seen = HashSet::<u32>::new();

    for (sum, num) in input {
        if seen.contains(&u32::try_from(TARGET as i32 - sum as i32).unwrap_or(0)) {
            return (sum - num) * num * (TARGET - sum);
        }
        seen.insert(num);
    }
    panic!("No answer found")
}


#[cfg(test)]
mod tests {
    use super::*;
    use common_helper::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day01.in")
    }

    #[test]
    fn task01_test() {
        let vec = input_data().into_iter().map(|num| num.parse().unwrap()).collect();
        let result = task01(vec);
        println!("task01: {}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data().into_iter().map(|num| num.parse().unwrap()).collect();
        let result = task02(vec);
        println!("task02: {}", result)
    }
}