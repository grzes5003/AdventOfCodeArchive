use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use futures::future::join_all;
use futures::{executor, FutureExt, join, StreamExt};

#[derive(Debug)]
struct Pattern {
    a: u8,
    b: u8,
    ch: char,
    passwd: String,
}

fn task01(input: Vec<Pattern>) -> u32 {
    executor::block_on(join_all(
        input.chunks(250)
            .map(|chunk| verify(chunk))
    ).then(|x| async move {
        x.into_iter()
            .reduce(|acc, next| acc + next)
            .unwrap()
    }))
}

async fn verify(input: &[Pattern]) -> u32 {
    input.into_iter()
        .map(|pattern|
            (pattern.a, pattern.b,
             pattern.passwd.chars()
                 .filter(|ch| *ch == pattern.ch)
                 .count() as u8
            )
        )
        .filter(|(a,b,count)|
            a <= count && count <= b
        )
        .count() as u32
}

#[cfg(test)]
mod tests {
    use futures::{executor, StreamExt};
    use super::*;
    use common_helper::parse;

    fn input_data() -> Vec<Pattern> {
        parse("resources/day02.in")
            .into_iter()
            .map(|line| {
                if let [_1, _2, _3] = line.split_whitespace().collect::<Vec<&str>>().as_slice() {
                    let a = _1.split('-').next().unwrap().parse().unwrap();
                    let b = _1.split('-').last().unwrap().parse().unwrap();
                    Pattern {
                        a,
                        b,
                        ch: _2.chars().next().unwrap(),
                        passwd: _3.to_string(),
                    }
                } else { panic!("cannot parse") }
            }).collect()
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        let result = task01(vec);
        println!("task01: {:?}", result)
    }
}