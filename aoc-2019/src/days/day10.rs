use std::collections::HashSet;
use crate::util::Solution;

type Loc = i32;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Asteroid {
    x: Loc,
    y: Loc,
}

struct Space {
    asteroids: Vec<Asteroid>,
    indirect: HashSet<(Asteroid, Asteroid)>
}

impl Space {
    fn new(asteroids: Vec<Asteroid>) -> Self {
        Self {
            asteroids,
            indirect: HashSet::new()
        }
    }

    fn count_visible(&mut self, a: &Asteroid) -> usize {
        let mut visible = 0;
        let (asteroids, indirect) = (&self.asteroids, &mut self.indirect);
        for b in asteroids.iter() {
            if a == b {
                continue;
            }
            if Self::is_visible_cached(a, b, asteroids, indirect) {
                visible += 1;
            }
        }
        visible
    }

    fn get_visible(&mut self, a: &Asteroid) -> Vec<Asteroid> {
        let (asteroids, indirect) = (&self.asteroids, &mut self.indirect);
        asteroids.iter()
            .filter(|asteroid| {
                if a == *asteroid {
                    return false;
                }
                Self::is_visible_cached(a, asteroid, asteroids, indirect)
            })
            .map(|asteroid| asteroid.clone())
            .collect()
    }

    fn is_visible_cached(a: &Asteroid, b: &Asteroid, asteroids: &Vec<Asteroid>, indirect: &mut HashSet<(Asteroid, Asteroid)>) -> bool {
        if indirect.get(&(*a,*b)).is_some() || indirect.get(&(*b,*a)).is_some() {
            return false;
        }

        for blocker in asteroids.iter() {
            if blocker == a || blocker == b {
                continue;
            }
            if Self::is_between(a, b, blocker) {
                indirect.insert((*a, *b));
                indirect.insert((*b, *a));
                return false;
            }
        }
        true
    }

    fn is_between(a: &Asteroid, b: &Asteroid, c: &Asteroid) -> bool {
        let len_ab = (a.x - b.x).abs() + (a.y - b.y).abs();
        let len_ac = (a.x - c.x).abs() + (a.y - c.y).abs();
        f64::atan2((a.y - b.y) as f64, (a.x - b.x) as f64)
            == f64::atan2((a.y - c.y) as f64, (a.x - c.x) as f64)
            && len_ab > len_ac
    }
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Vec<Asteroid>;
    type Output = Option<usize>;
    const DAY: &'a str = "Day10";

    fn part1(input: &Self::Input) -> Self::Output {
        let mut space = Space::new(input.clone());
        input.iter().map(|asteroid| {
            space.count_visible(asteroid)
        }).max()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        const TARGET: Asteroid = Asteroid { x: 22, y: 28 };
        let mut space = Space::new(input.clone());
        let mut visible = space.get_visible(&TARGET).into_iter()
            .map(|asteroid| {
                let angle = f64::atan2((TARGET.y - asteroid.y) as f64, (TARGET.x - asteroid.x) as f64) - std::f64::consts::PI / 2.0;
                let angle = if angle < 0.0 { angle + 2.0 * std::f64::consts::PI } else { angle };
                (angle, asteroid)
            }).collect::<Vec<_>>();
        visible.sort_by(|(angle, _), (angle2, _)| {
            angle.partial_cmp(angle2).unwrap()
        });
        if let Some((_, ast)) = visible.get(199) {
            return Some((ast.x * 100 + ast.y) as usize);
        }
        None
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.into_iter().enumerate()
            .map(|(idy, line)| {
                line.chars().enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(idx, _)| Asteroid { x: idx as Loc, y: idy as Loc })
                    .collect::<Vec<_>>()
            }).flatten()
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::parse;
    use super::*;

    #[test]
    fn basic_test_00() {
        let input = parse("input/day10_test.in");
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(8));
    }

    #[test]
    fn basic_test_01() {
        let input = parse("input/day10_basic.in");
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(33));
    }
}