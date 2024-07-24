use crt::DeckCards;
use aoko::standard::functions::fun::read_line;

fn main() {
    let mut round = 1;
    loop {
        let mut cards = DeckCards::new();
        println!("{}Round {}{round} {}started:{}", "\x1b[36m", "\x1b[35m", "\x1b[36m", "\x1b[0m");
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
