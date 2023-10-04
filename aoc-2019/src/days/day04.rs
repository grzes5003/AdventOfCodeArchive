use crate::err::InputError;
use crate::util::Solution;

type Num = u32;
pub struct Range {
    start: Num,
    end: Num,
}

pub struct Day;

impl Day {
    fn is_valid_num(passwd: &Num) -> bool {
        let mut passwd = passwd.to_string().into_bytes();
        let mut has_double = false;
        for i in 1..6 {
            if passwd[i] < passwd[i - 1] {
                return false;
            }
            if passwd[i] == passwd[i - 1] {
                has_double = true;
            }
        }
        has_double
    }

    fn is_valid_num_strict(passwd: &Num) -> bool {
        let mut passwd = passwd.to_string().into_bytes();
        let mut double = Vec::new();
        for i in 1..6 {
            if passwd[i] < passwd[i - 1] {
                return false;
            }
            if passwd[i] == passwd[i - 1] {
                double.push(passwd[i]);
            }
        }
        if double.len() == 0 {
            return false;
        }
        double.iter()
            .map(|&d| passwd.iter().filter(|&&e| e == d).count())
            .any(|c| c == 2)
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Result<Range, InputError>;
    type Output = Option<Num>;
    const DAY: &'a str = "Day04";

    fn part1(input: &Self::Input) -> Self::Output {
        let range = input.as_ref().ok()?;
        Some((range.start..=range.end)
            .filter(|passwd| Day::is_valid_num(passwd))
            .count() as Num)
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let range = input.as_ref().ok()?;
        Some((range.start..=range.end)
            .filter(|passwd| Day::is_valid_num_strict(passwd))
            .count() as Num)
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut iter = raw_input[0].split('-');
        let start = iter.next().unwrap().parse::<Num>()
            .map_err(|a| InputError::WrongFormat(a.to_string()))?;
        let end = iter.next().unwrap().parse::<Num>()
            .map_err(|a| InputError::WrongFormat(a.to_string()))?;
        Ok(Range { start, end })
    }
}