use std::cmp::Ordering;
use std::fmt::{Display, Formatter, self};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,Eq)]
pub struct Creature {
    name: String,
    initiative: usize,
    // used as a tie breaker, does not make an initiative of 15 suddenly beat an initiative of 21
    bonus: usize
}

impl<'a> Creature {
    pub fn new<T: ToString>(creature_name: T, init: usize) -> Self {
        Creature { name: creature_name.to_string(), initiative: init, bonus: 0 }
    }

    pub fn with_modifier<T: ToString>(creature_name: T, init: usize, modifier: usize) -> Self {
        Creature { name: creature_name.to_string(), initiative: init, bonus: modifier }
    }

    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn increment(&mut self, bonus: usize) {
        self.bonus += bonus;
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

    // parse from format of <name>, <init>
    // ignore everything else
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(", ").collect();
        if v.len() > 2 {
            return Err(CreatureError::TooFewArgs)
        }
        let init = match v[1].parse() {
            Ok(val) => val,
            Err(e) => return Err(CreatureError::Int(e)),
        };
        Ok(Creature::new(v[0], init))
    }
}

impl Display for Creature {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("{}", self.name))
    }
}
