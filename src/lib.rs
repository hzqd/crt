use aoko::{no_std::{algebraic::sum::{Map, MapExt}, pipelines::{pipe::Pipe, tap::Tap}}, standard::functions::ext::StdAnyExt};

pub struct DeckCards(Map<char, u8>);

const RANK_ORDER: [char; 15] = ['3', '4', '5', '6', '7', '8', '9', 'S', 'J', 'Q', 'K', 'A', '2', 'D', 'X'];

impl DeckCards {
    pub fn empty() -> DeckCards {
        DeckCards(Map::with_capacity(15))
    }

    pub fn new() -> DeckCards {
        DeckCards::empty().tap_mut(|cards| {
            RANK_ORDER
                .tap(|&o| o.into_iter().take(13).for_each(|r| cards.add_card(r, 4)))
                .into_iter().rev().take(2).for_each(|r| cards.add_card(r, 1))
        })
    }

    fn add_card(&mut self, rank: char, n: u8) {
        self.0.set(rank, n);
    }

    pub fn remove_card(&mut self, rank: char) {
        if let Some(count) = self.0.get_mut(&rank) {
            *count -= 1;
            if *count == 0 {
                self.0.delete(&rank);
            }
        } else {
            println!("Invalid Card: {rank}");
        }
    }

    fn colored_rank(n: u8) -> impl FnOnce(char) -> [char; 6] {
        move |rank| match n {
            4 => ['\x1b', '[', '3', '1', 'm', rank], // 红
            3 => ['\x1b', '[', '3', '3', 'm', rank], // 橙 in Windows Terminal
            2 => ['\x1b', '[', '3', '2', 'm', rank], // 绿
            1 => ['\x1b', '[', '3', '7', 'm', rank], // 白
            n => panic!("internal card number error: {n}"),
        }
    }
    pub fn print(&self) {
        self.0
            .iter()
            .rev()
            .flat_map(|&(rank, n)| rank.pipe(DeckCards::colored_rank(n)))
            .chain("\x1b[0m".chars())
            .collect::<String>()
            .echo();
    }
}
