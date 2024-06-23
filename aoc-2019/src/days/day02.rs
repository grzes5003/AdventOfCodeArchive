use crate::util::Solution;

type Num = u32;

pub struct Day;

impl Day {
    fn calculate(input: &Vec<Num>) -> Num {
        let mut input = input.clone();
        let mut ptr = 0;

        while ptr + 4 <= input.len() {
            let opcode = input[ptr] as usize;
            let a = input[ptr + 1] as usize;
            let b = input[ptr + 2] as usize;
            let target = input[ptr + 3] as usize;

            let val = match opcode {
                1 => input[a] + input[b],
                2 => input[a] * input[b],
                99 => break,
                _ => panic!("Invalid opcode: {}", opcode)
            };
            input[target] = val;
            ptr += 4;
        }
        input[0]
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Vec<Num>;
    type Output = Option<Num>;
    const DAY: &'a str = "Day02";

    fn part1(input: &Self::Input) -> Self::Output {
        let mut input = input.clone();
        input[1] = 12;
        input[2] = 2;
        Some(Self::calculate(&input))
    }

    fn part2(input: &Self::Input) -> Self::Output {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut input = input.clone();
                input[1] = noun;
                input[2] = verb;
                if Self::calculate(&input) == 19690720 {
                    return Some(100 * noun + verb);
                }
            }
        }
        None
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.get(0).unwrap()
            .split(",")
            .map(|s| s.parse::<Num>().unwrap())
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::bench_day;
    use crate::util::parse;

    #[test]
    fn test_part1() {
        let input = parse("input/day02_test.in");
        let input = Day::parse_input(&input);
        assert_eq!(Day::calculate(&input), 3500);
    }

    #[test]
    fn test_minimal() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(Day::calculate(&input), 30);
    }

    #[test]
    fn test_minimal2() {
        let input = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(Day::calculate(&input), 2);
    }

    bench_day!(Day);
}