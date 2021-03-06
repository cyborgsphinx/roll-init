use std::cmp::Ordering;
use std::fmt::{Display, Formatter, self};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Eq)]
pub struct Creature {
    name: String,
    initiative: usize,
    // used as a tie breaker, does not make an initiative of 15 suddenly beat an initiative of 21
    bonus: Option<isize>,
}

impl<'a> Creature {
    pub fn new<T: ToString>(creature_name: T, init: usize) -> Self {
        Creature { name: creature_name.to_string(), initiative: init, bonus: None }
    }

    pub fn with_modifier<T: ToString>(creature_name: T, init: usize, modifier: isize) -> Self {
        Creature { name: creature_name.to_string(), initiative: init, bonus: Some(modifier) }
    }

    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn increment(&mut self, bonus: isize) {
        self.bonus = Some(self.bonus.unwrap_or(0) + bonus);
    }
}

impl PartialEq for Creature {
    fn eq(&self, other: &Self) -> bool {
        self.initiative == other.initiative && self.bonus == other.bonus
    }
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Creature {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.initiative.cmp(&other.initiative) {
            Ordering::Equal => self.bonus.cmp(&other.bonus),
            ord @ _ => ord,
        }
    }
}

#[derive(Debug)]
pub enum CreatureError {
    Int(ParseIntError),
    TooFewArgs,
}

impl FromStr for Creature {
    type Err = CreatureError;

    // parse from format of <name>,? <init>(,? <modifier>)?
    // ignore everything else
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split_whitespace().collect();
        if v.len() < 2 {
            return Err(CreatureError::TooFewArgs)
        }
        let name = v[0].trim_matches(',');
        let init = match v[1].trim_matches(',').parse() {
            Ok(val) => val,
            Err(e) => return Err(CreatureError::Int(e)),
        };
        if v.len() > 2 {
            let modifier = match v[2].trim_matches(',').parse() {
                Ok(val) => val,
                Err(e) => return Err(CreatureError::Int(e)),
            };
            Ok(Creature::with_modifier(name, init, modifier))
        } else {
            Ok(Creature::new(name, init))
        }
    }
}

impl Display for Creature {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.bonus {
            Some(val) => f.write_fmt(format_args!("{}, {}, {}", self.name, self.initiative, val)),
            None => f.write_fmt(format_args!("{}, {}", self.name, self.initiative)),
        }
    }
}
