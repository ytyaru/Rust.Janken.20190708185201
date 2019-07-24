/*
 * じゃんけんゲーム。
 * CreatedAt: 2019-07-08
 */
use rand::{distributions::{Distribution, Standard},Rng};
use std::io::Write;
use std::io::stdout;

#[derive(Debug,PartialEq)]
pub enum Result { Win, Lose, Draw }
impl std::fmt::Display for crate::Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            crate::Result::Win => write!(f, "Win!!"),
            crate::Result::Lose => write!(f, "Lose..."),
            crate::Result::Draw => write!(f, "Draw"),
        }
    }
}
#[derive(Debug,PartialEq)]
pub enum Hand { Rock, Scissors, Paper }
impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Hand::Rock => write!(f, "✊"),
            Hand::Scissors => write!(f, "✌"),
            Hand::Paper => write!(f, "✋"),
        }
    }
}
impl From<u32> for Hand {
    fn from(hand: u32) -> Hand {
        match hand {
            1 => Hand::Rock,
            2 => Hand::Scissors,
            3 => Hand::Paper,
            _ => panic!("手は1〜3のいずれかで指定してください。")
        }
    }
}
impl Distribution<Hand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hand {
        Hand::from(rng.gen_range(1, 4))
    }
}
impl Hand {
    pub fn jadge(&self, opponent: Hand) -> crate::Result {
        if *self == Hand::Rock && opponent == Hand::Scissors { crate::Result::Win }
        else if *self == Hand::Rock && opponent == Hand::Paper { crate::Result::Lose }
        else if *self == Hand::Scissors && opponent == Hand::Rock { crate::Result::Lose }
        else if *self == Hand::Scissors && opponent == Hand::Paper { crate::Result::Win }
        else if *self == Hand::Paper && opponent == Hand::Rock { crate::Result::Win }
        else if *self == Hand::Paper && opponent == Hand::Scissors { crate::Result::Lose }
        else { crate::Result::Draw }
    }
}

