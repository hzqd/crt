use aoko::standard::functions::fun::read_line;
use crt::DeckCards;

fn main() {
    let mut round = 1;
    loop {
        let mut cards = DeckCards::new();
        println!("{}Round {}{round} {}started:{}", "\x1b[32m", "\x1b[34m", "\x1b[32m", "\x1b[0m");
        loop {
            match read_line().to_uppercase().trim_end() {
                "R" => { break; },
                "N" => { round += 1; break; },
                input => input.chars().for_each(|rank| cards.remove_card(rank)),
            };
            cards.print()
        }
    }
}