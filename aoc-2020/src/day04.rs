use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::ops::Range;
use futures::StreamExt;

#[derive(Debug, Clone)]
struct ParseError;
impl Error for ParseError {}
#[derive(Debug, Clone)]
struct ValueError;
impl Error for ValueError {}

type Num = u32;
trait RangeValidator {
    fn range_rule(self, range: Range<Num>) -> Result<Self, ValueError> where Self: Sized;
}

trait StringValidator {
    fn height_validator(self) -> Result<Num, ValueError>;
    fn hair_validator(self) -> Result<Self, ValueError> where Self: Sized;
    fn eye_validator(self) -> Result<Self, ValueError> where Self: Sized;
    fn pid_validator(self) -> Result<Self, ValueError> where Self: Sized;
}

impl RangeValidator for Num {
    fn range_rule(self, range: Range<Num>) -> Result<Self, ValueError> {
        if range.contains(&self) {return Ok(self)}; Err(ValueError)
    }
}

impl StringValidator for String {
    fn height_validator(self) -> Result<Num, ValueError> {
        match self {
            val if val.ends_with("cm") => {
                val.strip_suffix("cm").unwrap()
                    .parse::<Num>().or(Err(ValueError))?
                    .range_rule((150..194))
            },
            val if val.ends_with("in") =>
                val.strip_suffix("in").unwrap()
                    .parse::<Num>().or(Err(ValueError))?
                    .range_rule((59..77)),
            _ => Err(ValueError)
        }
    }

    fn hair_validator(self) -> Result<Self, ValueError> {
        if let Some(foll) = self.strip_prefix('#') {
            if foll.chars().all(
                |ch| (b'a'..=b'f').contains(&(ch as u8)) ||
                    (b'0'..=b'9').contains(&(ch as u8))
            ) && foll.len() == 6 {
                return Ok(self)
            }
        }
        Err(ValueError)
    }

    fn eye_validator(self) -> Result<Self, ValueError> where Self: Sized {
        if vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().any(|val| val.to_string() == self) {
            return Ok(self)
        }
        Err(ValueError)
    }

    fn pid_validator(self) -> Result<Self, ValueError> where Self: Sized {
        if self.chars().all(char::is_numeric) && self.len() == 9 {
            return Ok(self)
        }
        Err(ValueError)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid parse value")
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid value")
    }
}

struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: u32,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    pub fn from(s: String) -> Result<Passport, Box<dyn Error>> {
        let mut seq: HashMap<String, String> = HashMap::new();
        let res: Result<Vec<_>, Box<ParseError>> = s.split_whitespace().map(|token| {
            if let [tk, val] = *token.split(':').collect::<Vec<&str>>().as_slice() {
                Ok((tk, val))
            } else { Err(Box::new(ParseError)) }
        }).map(|z| z).collect();

        if let Ok(r) = res {
            r.into_iter()
                .for_each(|(key, val)| { seq.insert(key.to_string(), val.to_string()); });
            return Passport::visit_seq(seq)
        }
        Err(Box::new(ParseError))
    }

    fn visit_seq(mut seq: HashMap<String, String>) -> Result<Passport, Box<dyn Error>> {
        let byr = seq.get("byr").ok_or(ParseError)?.parse::<Num>()?.range_rule((1920..2003))?;
        let eyr = seq.get("eyr").ok_or(ParseError)?.parse::<Num>()?.range_rule((2020..2031))?;
        let hgt = seq.get("hgt").ok_or(ParseError)?.clone().height_validator()?;
        let iyr = seq.get("iyr").ok_or(ParseError)?.parse::<Num>()?.range_rule((2010..2021))?;
        let hcl = seq.get("hcl").ok_or(ParseError)?.clone().hair_validator()?;
        let ecl = seq.get("ecl").ok_or(ParseError)?.clone().eye_validator()?;
        let pid = seq.get("pid").ok_or(ParseError)?.clone().pid_validator()?;
        let cid = seq.get("cid").or(None).map(|v| v.clone());
        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        })
    }
}


fn task01(input: Vec<String>) -> u32 {
    input.into_iter()
        .map(Passport::from)
        .filter(|res| res.is_ok())
        .count() as u32
}

#[cfg(test)]
mod tests {
    use futures::{executor, StreamExt};
    use super::*;
    use common_helper::parse;

    fn input_data() -> Vec<String> {
        let mut res = Vec::new();
        let mut tmp = Vec::new();
        for line in parse("resources/day04.in") {
            match line.is_empty() {
                true => { res.push(tmp.clone()); tmp.clear(); },
                false => tmp.push(line)
            }
        }
        res.into_iter().map(|v|
            v.into_iter().map(
                |element| element.replace('\n',"")
            ).collect::<Vec<String>>().join(" ")
        ).collect()
    }

    #[test]
    fn task01_test() {
        let vec = input_data();
        println!("{:?}", vec);
        let result = task01(vec);
        println!("task01: {:?}", result)
    }
}