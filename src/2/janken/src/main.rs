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
        if player != 1 && player !=2 && player != 3 { break; }
        let npc = npc();
        let result = jadge(player, npc);
        println!("{} {}", hand(player), hand(npc));
        show_result(result);
    }
}
fn read_stdin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn npc() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 4)
}
// 0:draw, 1:lose, 2:win
fn jadge(player:u32, npc:u32) -> u32 {
    if      player == 1 && npc == 2 { 2 }
    else if player == 1 && npc == 3 { 1 }
    else if player == 2 && npc == 1 { 1 }
    else if player == 2 && npc == 3 { 2 }
    else if player == 3 && npc == 1 { 2 }
    else if player == 3 && npc == 2 { 1 }
    else { 0 }
}
// 1:✊, 2:✌, 3:✋
fn hand(hand:u32) -> char {
    match hand {
        1 => '✊',
        2 => '✌',
        3 => '✋',
        _ => panic!("手は1,2,3のいずれかのみ。"),
    }
}
fn show_result(result: u32) {
    if result == 1 { println!("Lose..."); }
    else if result == 2 { println!("Win!!"); }
    else { println!("Draw"); }
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
    fn from(&self, hand: i32) -> Hand {
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

