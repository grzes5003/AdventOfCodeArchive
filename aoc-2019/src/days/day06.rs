use std::cmp::min;
use std::collections::{HashMap, HashSet};
use crate::err::InputError;
use crate::util::Solution;


#[derive(Debug)]
pub struct Orbits {
    orbits: HashMap<String, Vec<String>>,
}

impl Orbits {

    fn from_str_vec(s: &Vec<String>) -> Result<Self, InputError> {
        let mut orbits = HashMap::new();
        for line in s {
            let mut line = line.split(')');
            let center = line.next().ok_or(InputError::WrongFormat("No left".to_string()))?;
            let orbiter = line.next().ok_or(InputError::WrongFormat("No right".to_string()))?;
            let orb = orbits.entry(center.to_string()).or_insert(Vec::new());
            orb.push(orbiter.to_string());
        }
        Ok(Orbits { orbits })
    }

    fn from_str_vec_exp(s: &Vec<String>) -> Result<Self, InputError> {
        let mut orbits = HashMap::new();
        for line in s {
            let mut line = line.split(')');
            let center = line.next().ok_or(InputError::WrongFormat("No left".to_string()))?;
            let orbiter = line.next().ok_or(InputError::WrongFormat("No right".to_string()))?;
            let orb = orbits.entry(center.to_string()).or_insert(Vec::new());
            orb.push(orbiter.to_string());
            let orb = orbits.entry(orbiter.to_string()).or_insert(Vec::new());
            orb.push(center.to_string());
        }
        Ok(Orbits { orbits })
    }
}

impl Orbits {
    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut visited = HashSet::new();
        let mut buff = vec![("COM".to_string(), 0usize)];

        while let Some((node, depth)) = buff.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());
            checksum += depth.checked_sub(1).unwrap_or(0)
                    + min(depth, 1);

            for child in self.orbits
                .get(node.as_str())
                .unwrap_or(&Vec::<String>::new()) {
                buff.push((child.to_owned(), depth + 1));
            }
        }
        checksum
    }

    fn path_len(&self, start: &str, end: &str) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut buff = vec![(start.to_string(), 0usize)];

        while let Some((node, depth)) = buff.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());
            if node == end {
                return Some(depth);
            }
            for child in self.orbits
                .get(node.as_str())
                .unwrap_or(&Vec::<String>::new()) {
                buff.push((child.to_owned(), depth + 1));
            }
        }
        None
    }
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Vec<String>;
    type Output = Option<usize>;
    const DAY: &'a str = "Day06";

    fn part1(input: &Self::Input) -> Self::Output {
        let input = Orbits::from_str_vec(input).ok()?;
        Some(input.checksum())
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let input = Orbits::from_str_vec_exp(input).ok()?;
        Some(input.path_len("YOU", "SAN")? - 2)
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        // transparent parsing
        raw_input.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::parse;

    #[test]
    fn test_sample_input() {
        let input = parse("input/day06_test.in");
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(42));
    }
}