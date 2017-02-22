extern crate rand;

use std::fmt;
use game::player::Player;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Type {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Type {
    pub fn get_beats(&self) -> Vec<Type> {
        match self {
            &Type::ROCK => vec![ Type::SCISSORS ],
            &Type::PAPER => vec![ Type::ROCK ],
            &Type::SCISSORS => vec![ Type::PAPER ],
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Play<'a> {
    pub choice: Type,
    pub player: &'a mut Player,
}

pub fn parse_type(name: String) -> Option<Type> {
    for typ in vec![ Type::ROCK, Type::PAPER, Type::SCISSORS ] {
        if typ.to_string().to_lowercase() == name.to_lowercase() {
            return Some(typ);
        }
    }
    None
}

pub fn random_play() -> Type {
    match rand::thread_rng().gen_range(0, 3) {
        0 => Type::ROCK,
        1 => Type::PAPER,
        2 => Type::SCISSORS,
        _ => panic!("Unable to generate random play."),
    }
}