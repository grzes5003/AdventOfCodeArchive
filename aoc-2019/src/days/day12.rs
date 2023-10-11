use std::collections::HashSet;
use std::str::FromStr;
use crate::err::InputError;
use crate::util::Solution;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Moon {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

impl Moon {
    fn update(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn update_vel(&mut self, other: &Moon) {
        if self.pos.0 < other.pos.0 {
            self.vel.0 += 1;
        } else if self.pos.0 > other.pos.0 {
            self.vel.0 -= 1;
        }
        if self.pos.1 < other.pos.1 {
            self.vel.1 += 1;
        } else if self.pos.1 > other.pos.1 {
            self.vel.1 -= 1;
        }
        if self.pos.2 < other.pos.2 {
            self.vel.2 += 1;
        } else if self.pos.2 > other.pos.2 {
            self.vel.2 -= 1;
        }
    }

    fn potential_energy(&self) -> usize {
        (self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()) as usize
    }

    fn kinetic_energy(&self) -> usize {
        (self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()) as usize
    }

    fn get_axis(&self, axis: usize) -> (i32, i32) {
        match axis {
            0 => (self.pos.0, self.vel.0),
            1 => (self.pos.1, self.vel.1),
            2 => (self.pos.2, self.vel.2),
            _ => panic!("Invalid axis")
        }
    }
}

impl FromStr for Moon {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = (0, 0, 0);
        for (idx, part) in s.trim_matches(|c| c == '<' || c == '>').split(',').enumerate() {
            let mut part = part.split('=');
            let val = part.nth(1).ok_or(InputError::WrongFormat("Invalid input".to_string()))?
                .parse::<i32>().map_err(|e| InputError::WrongFormat(e.to_string()))?;
            match idx {
                0 => pos.0 = val,
                1 => pos.1 = val,
                2 => pos.2 = val,
                _ => return Err(InputError::WrongFormat("Invalid input".to_string()))
            }
        }
        Ok(Self { pos, vel: (0, 0, 0) })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct System {
    moons: Vec<Moon>,
}

impl System {
    fn step(&mut self) {
        let moons = self.moons.clone();
        for moon in self.moons.iter_mut() {
            for other in moons.iter() {
                if moon.pos == other.pos {
                    continue;
                }
                moon.update_vel(&other);
            }
            moon.update();
        }
    }

    fn get_axis(&self, axis: usize) -> HashSet<(i32, i32)> {
        self.moons.iter()
            .map(|moon| moon.get_axis(axis))
            .collect()
    }
}

fn cmp_axis(a: HashSet<(i32, i32)>, b: HashSet<(i32, i32)>) -> bool {
    let pos = b.iter().map(|(v, _)| v).collect::<HashSet<_>>();
    let f = a.iter().all(|(x, _)| pos.contains(x));
    (b.iter().map(|(_, v)| v).all(|v| v == &0)
        || a.iter().map(|(_, v)| v).all(|v| v == &0))
        && f
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Vec<Moon>;
    type Output = usize;
    const DAY: &'a str = "Day12";

    fn part1(input: &Self::Input) -> Self::Output {
        let mut system = System { moons: input.to_owned() };
        for _ in 0..1000 {
            system.step();
        }
        system.moons.iter()
            .map(|moon| moon.potential_energy() * moon.kinetic_energy())
            .sum::<usize>()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut system = System { moons: input.to_owned() };
        let mut steps = 0;
        let mut old_system = system.clone();
        let mut smallest = [i32::MAX; 3];
        loop {
            system.step();
            steps += 1;
            if smallest[0] == i32::MAX && cmp_axis(system.get_axis(0), old_system.get_axis(0)) {
                smallest[0] = steps;
            }
            if smallest[1] == i32::MAX && cmp_axis(system.get_axis(1), old_system.get_axis(1)) {
                smallest[1] = steps;
            }
            if smallest[2] == i32::MAX && cmp_axis(system.get_axis(2), old_system.get_axis(2)) {
                smallest[2] = steps;
            }
            if smallest[0] != i32::MAX && smallest[1] != i32::MAX && smallest[2] != i32::MAX {
                break;
            }
            old_system = system.clone();
        }
        smallest.iter()
            .fold(1, |acc, &x| lcm(acc, x as usize)) * 2
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.into_iter()
            .map(|line| line.parse::<Moon>().unwrap())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<Moon> {
        vec![
            Moon { pos: (-1, 0, 2), vel: (0, 0, 0) },
            Moon { pos: (2, -10, -7), vel: (0, 0, 0) },
            Moon { pos: (4, -8, 8), vel: (0, 0, 0) },
            Moon { pos: (3, 5, -1), vel: (0, 0, 0) },
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day::part1(&input()), 179);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day::part2(&input()), 2772);
    }
}