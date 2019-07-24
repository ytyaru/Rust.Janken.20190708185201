/*
 * じゃんけんゲーム。
 * CreatedAt: 2019-07-08
 */
use rand::Rng;
use std::io::Write;
use std::io::stdout;
fn main() {
    loop {
        println!("じゃんけんゲーム");
        print!("1:✊ 2:✌ 3:✋> ");
        stdout().flush();
        let player: u32 = read_stdin();
        let player = Hand::Rock.from(player);
        let npc = Hand::Rock.random();
        let res = Hand::Rock.jadge(player, npc);
        crate::Result::Win.show(res);
    }
}
fn read_stdin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[derive(Debug,PartialEq)]
enum Result { Win, Lose, Draw }
impl crate::Result {
    fn show(&self, hand: crate::Result) {
        if hand == crate::Result::Win { println!("Win!!"); }
        else if hand == crate::Result::Lose { println!("Lose..."); }
        else { println!("Draw"); }
    }
}
#[derive(Debug,PartialEq)]
enum Hand { Rock, Scissors, Paper }
impl Hand {
    fn from(&self, hand: u32) -> Hand {
        match hand {
            1 => Hand::Rock,
            2 => Hand::Scissors,
            3 => Hand::Paper,
            _ => panic!("手は1〜3のいずれかで指定してください。")
        }
    }
    fn random(&self) -> Hand {
        let mut rng = rand::thread_rng();
        let hand = rng.gen_range(1, 4);
        self.from(hand)
    }
    fn jadge(&self, pc: Hand, npc: Hand) -> crate::Result {
        if pc == Hand::Rock && npc == Hand::Scissors { crate::Result::Win }
        else if pc == Hand::Rock && npc == Hand::Paper { crate::Result::Lose }
        else if pc == Hand::Scissors && npc == Hand::Rock { crate::Result::Lose }
        else if pc == Hand::Scissors && npc == Hand::Paper { crate::Result::Win }
        else if pc == Hand::Paper && npc == Hand::Rock { crate::Result::Win }
        else if pc == Hand::Paper && npc == Hand::Scissors { crate::Result::Lose }
        else { crate::Result::Draw }
    }
    fn show(&self, hand: Hand) -> char {
        if hand == Hand::Rock { '✊' }
        else if hand == Hand::Rock { '✌' }
        else { '✋' }
    }
}

