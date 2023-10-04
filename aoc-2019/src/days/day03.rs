use std::str::FromStr;
use crate::err::InputError;
use crate::util::Solution;


type Num = i32;
type Point = (Num, Num);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    dir: Direction,
    len: Num,
}

pub struct Line(Vec<Instruction>);

impl Line {
    fn walk(&self) -> impl Iterator<Item=(Point, Point)> + '_ {
        let mut start = (0, 0);
        self.0.iter()
            .map(move |instr| {
                let mut end = start;
                match instr.dir {
                    Direction::Up => end.1 += instr.len,
                    Direction::Down => end.1 -= instr.len,
                    Direction::Left => end.0 -= instr.len,
                    Direction::Right => end.0 += instr.len,
                }
                let result = (start, end);
                start = end;
                result
            })
    }
}

impl FromStr for Instruction {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = s.split_at(1);
        let dir = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            ch => return Err(InputError::WrongFormat(ch.to_owned()))
        };
        let len = len.parse::<Num>().map_err(|_| InputError::WrongFormat(len.to_owned()))?;
        return Ok(Instruction { dir, len });
    }
}

pub struct Day;

impl Day {
    fn manhattan(p: Point) -> Num {
        p.0.abs() + p.1.abs()
    }

    fn crossing_point(l1: &(Point, Point), l2: &(Point, Point)) -> Option<Point> {
        let (p1, p2) = l1;
        let (p3, p4) = l2;
        if p1.1 == p2.1 {
            let x = p3.0;
            let y = p1.1;
            let (min1, max1) = (p1.0.min(p2.0), p1.0.max(p2.0));
            let (min2, max2) = (p3.1.min(p4.1), p3.1.max(p4.1));
            if x >= min1 && x <= max1 && y >= min2 && y <= max2 {
                return Some((x, y));
            }
        } else if p1.0 == p2.0 {
            let x = p1.0;
            let y = p3.1;
            let (min1, max1) = (p1.1.min(p2.1), p1.1.max(p2.1));
            let (min2, max2) = (p3.0.min(p4.0), p3.0.max(p4.0));
            if y >= min1 && y <= max1 && x >= min2 && x <= max2 {
                return Some((x, y));
            }
        }
        None
    }

    fn len((a, b): &(Point, Point)) -> Num {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Result<(Line, Line), InputError>;
    type Output = Option<Num>;
    const DAY: &'a str = "Day03";

    fn part1(input: &Self::Input) -> Self::Output {
        let (line1, line2) = input.as_ref().ok()?;
        let (horizontal, vertical): (Vec<_>, Vec<_>) = line1.walk()
            .partition(|(p1, p2)| p1.0 == p2.0);
        line2.walk()
            .map(|(a, b)| if a.0 == b.0 {
                ((a, b), Box::new(&vertical))
            } else {
                ((a, b), Box::new(&horizontal))
            })
            .map(|(line, lines)| {
                lines.iter()
                    .map(move |line2| Day::crossing_point(&line, line2))
                    .flatten()
                    .filter(|point| *point != (0, 0))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|p1| Day::manhattan(p1))
            .min()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let (line1, line2) = input.as_ref().ok()?;
        let walk_line1 = line1.walk()
            .scan(0, |acc, line| {
                *acc += Day::len(&line);
                Some(*acc)
            }).zip(line1.walk()).collect::<Vec<_>>();

        let mut walk_line2 = line2.walk();
        let mut line2_len = 0;
        while let Some(line_a) = walk_line2.next() {
            let first_intersection = walk_line1.iter()
                .map(|(len, line_b)| (len, line_b, Day::crossing_point(&line_a, line_b)))
                .find(|(_, _, inter)| inter.is_some());
            if let Some((len, line_b, point)) = first_intersection {
                if point.unwrap() != (0, 0) {
                    return Some(line2_len + Day::len(&(line_a.0, point.unwrap()))
                        + len - Day::len(line_b) + Day::len(&(line_b.0, point.unwrap())));
                }
            }
            line2_len += Day::len(&line_a);
        }
        None
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut lines = raw_input.iter().map(|s| {
            let instrs = s.split(",")
                .map(|s| s.parse::<Instruction>())
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Line(instrs))
        });
        Ok(
            (lines.next().ok_or(InputError::WrongFormat("".to_string()))??,
             lines.next().ok_or(InputError::WrongFormat("".to_string()))??)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::bench_day;
    use crate::util::parse;

    #[test]
    fn test_input1() {
        let input = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
            "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(159));
    }

    #[test]
    fn test_input2() {
        let input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(135));
    }

    #[test]
    fn test2_input1() {
        let input = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
            "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part2(&input), Some(610));
    }

    #[test]
    fn test2_input2() {
        let input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part2(&input), Some(410));
    }

    #[test]
    fn test_minimal() {
        let input = vec![
            "R8,U5,L5,D3".to_string(),
            "U7,R6,D4,L4".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(6));
    }

    #[test]
    fn test2_minimal() {
        let input = vec![
            "R8,U5,L5,D3".to_string(),
            "U7,R6,D4,L4".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part2(&input), Some(30));
    }


    bench_day!(Day);
}

