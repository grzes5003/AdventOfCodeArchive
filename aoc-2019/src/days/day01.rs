use crate::util::{parse, Solution};

type Num = u32;
pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Vec<Num>;
    type Output = Num;
    const DAY: &'a str = "Day01";

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().map(|n| n / 3 - 2).sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.iter()
            .map(|n| {
                let mut sum = 0;
                let mut fuel = *n;
                while fuel > 0 {
                    fuel = (fuel / 3).checked_sub(2).unwrap_or(0);
                    sum += fuel;
                }
                sum
            }).sum()
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.iter().map(|s| s.parse::<Num>().unwrap()).collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = parse("input/day01.in");
        let input = Day::parse_input(&input);
        b.iter(|| Day::part1(&input));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = parse("input/day01.in");
        let input = Day::parse_input(&input);
        b.iter(|| Day::part2(&input));
    }
}