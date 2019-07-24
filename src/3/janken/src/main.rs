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
        let player = janken::Hand::from(player);
        let npc: janken::Hand = rand::random();
        println!("{} {}", player, npc);
        let res = player.jadge(npc);
        println!("{}", res);
    }
}
fn read_stdin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

