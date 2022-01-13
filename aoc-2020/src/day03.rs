use futures::StreamExt;

fn task01(input: Vec<String>) -> u32 {
    encounter(&input, (3,1))
}

fn task02(input: Vec<String>) -> u32 {
    vec![(1,1),(3,1),(5,1),(7,1),(1,2)].into_iter()
        .map(|angle| encounter(&input,angle))
        .fold(1, |acc, curr| acc * curr)
}

fn encounter(input: &Vec<String>, angle: (usize,usize)) -> u32 {
    input.iter().step_by(angle.1).enumerate().map(|(idx, line)| {
        let ch = line.chars().cycle().nth(angle.0 * idx).unwrap();
        if ch == '#' {1} else {0}
    }).sum()
}

#[cfg(test)]
mod tests {
    use futures::{executor, StreamExt};
    use super::*;
    use common_helper::parse;

    fn input_data() -> Vec<String> {
        parse("resources/day03.in")
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {:?}", result)
    }

    #[test]
    fn task02_test() {
        let vec = input_data();
        let result = task02(vec);
        println!("task02: {:?}", result)
    }
}