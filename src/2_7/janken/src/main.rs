/*
 * じゃんけんゲーム。
 * CreatedAt: 2019-07-08
 */
use rand::{distributions::{Distribution, Standard},Rng};
use std::io::Write;
use std::io::stdout;
use rand::seq::SliceRandom;
fn main() {
    loop {
        println!("じゃんけんゲーム");
        print!("1:✊ 2:✌ 3:✋> ");
        stdout().flush();
        let player: u32 = read_stdin();
        let player = Hand::from(player);
        let npc: Hand = rand::random();
        println!("{} {}", player, npc);
//        let res = Hand::Rock.jadge(player, npc);
        let res = player.jadge(npc);
        println!("{}", res);
    }
}
fn read_stdin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[derive(Debug,PartialEq)]
enum Result { Win, Lose, Draw }
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
enum Hand { Rock, Scissors, Paper }
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
    /*
    fn jadge(&self, pc: Hand, npc: Hand) -> crate::Result {
        if pc == Hand::Rock && npc == Hand::Scissors { crate::Result::Win }
        else if pc == Hand::Rock && npc == Hand::Paper { crate::Result::Lose }
        else if pc == Hand::Scissors && npc == Hand::Rock { crate::Result::Lose }
        else if pc == Hand::Scissors && npc == Hand::Paper { crate::Result::Win }
        else if pc == Hand::Paper && npc == Hand::Rock { crate::Result::Win }
        else if pc == Hand::Paper && npc == Hand::Scissors { crate::Result::Lose }
        else { crate::Result::Draw }
    }
    */
    fn jadge(&self, opponent: Hand) -> crate::Result {
        if *self == Hand::Rock && opponent == Hand::Scissors { crate::Result::Win }
        else if *self == Hand::Rock && opponent == Hand::Paper { crate::Result::Lose }
        else if *self == Hand::Scissors && opponent == Hand::Rock { crate::Result::Lose }
        else if *self == Hand::Scissors && opponent == Hand::Paper { crate::Result::Win }
        else if *self == Hand::Paper && opponent == Hand::Rock { crate::Result::Win }
        else if *self == Hand::Paper && opponent == Hand::Scissors { crate::Result::Lose }
        else { crate::Result::Draw }
    }
}

