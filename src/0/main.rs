/*
 * じゃんけんゲーム。
 * CreatedAt: 2019-07-08
 */
use std::io::Write;
use std::io::stdout;
fn main() {
    loop {
        println!("じゃんけんゲーム");
        print!("1:✊ 2:✌ 3:✋> ");
        stdout().flush();
        let player: u32 = read_stdin();
        if player != 1 && player !=2 && player != 3 { break; }
        let npc = 1;
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
