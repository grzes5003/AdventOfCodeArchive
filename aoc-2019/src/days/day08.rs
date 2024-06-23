use std::fmt;
use std::fmt::Formatter;
use crate::util::Solution;

struct Img(Vec<Vec<char>>);

pub struct Day;

const WH: (usize, usize) = (25, 6);

impl Day {
    fn parse_layers(input: &str) -> Vec<Vec<char>> {
        let v = input.chars().collect::<Vec<_>>();
        v.chunks(WH.0 * WH.1)
            .map(|chunk| {
                chunk.to_vec()
            }).collect::<Vec<Vec<_>>>()
    }

    fn flatten(layers: &Vec<Vec<char>>) -> Img {
        let mut result = vec![vec!['2'; WH.0]; WH.1];
        let len = WH.0 * WH.1;
        for idx in 0..len {
            for layer in layers {
                if layer[idx] != '2' {
                    result[idx / WH.0][idx % WH.0] = layer[idx];
                    break;
                }
            }
        }
        Img(result)
    }
}

impl fmt::Debug for Img {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for c in row {
                write!(f, "{}", if *c == '1' { '█' } else { ' ' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<'a> Solution<'a> for Day {
    type Input = String;
    type Output = usize;
    const DAY: &'a str = "Day07";

    fn part1(input: &Self::Input) -> Self::Output {
        let layers = Self::parse_layers(input);
        let result = layers.iter()
            .min_by(|layer, layer2|
                layer.iter()
                    .filter(|&&c| c == '0').count()
                    .cmp(&layer2.iter().filter(|&&c| c == '0').count())
            ).unwrap();
        result.iter().filter(|&&c| c == '1').count() * result.iter().filter(|&&c| c == '2').count()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let layers = Self::parse_layers(input);
        let img = Self::flatten(&layers);
        println!("{:?}", img);
        0
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.get(0)
            .unwrap().to_owned()
    }
}