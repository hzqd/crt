use itertools::Itertools;
use std::collections::HashMap;
use aoko::{no_std::pipelines::{pipe::Pipe, tap::Tap}, standard::functions::ext::StdAnyExt};

pub struct DeckCards(HashMap<char, u8>);

const RANK_ORDER: [char; 15] = ['3', '4', '5', '6', '7', '8', '9', 'S', 'J', 'Q', 'K', 'A', '2', 'X', 'D'];

impl DeckCards {
    pub fn new() -> DeckCards {
        DeckCards(HashMap::new()).tap_mut(|cards| {
            RANK_ORDER
                .tap(|&o| o.into_iter().take(13).for_each(|r| cards.add_card(r, 4)))
                .into_iter().rev().take(2).for_each(|r| cards.add_card(r, 1))
        })
    }

    fn add_card(&mut self, rank: char, n: u8) {
        *self.0.entry(rank).or_insert(0) += n;
    }

    pub fn remove_card(&mut self, rank: char) {
        if let Some(count) = self.0.get_mut(&rank) {
            *count -= 1;
            if *count == 0 {
                self.0.remove(&rank);
            }
        } else {
            println!("Invalid Card: {rank}");
        }
    }

    fn colored_rank(n: u8) -> impl FnOnce(char) -> [char; 6] {
        move |rank| match n {
            4 => ['\x1b', '[', '3', '1', 'm', rank], // 红
            3 => ['\x1b', '[', '3', '5', 'm', rank], // 紫
            2 => ['\x1b', '[', '3', '3', 'm', rank], // 橙色 in Windows Terminal
            1 => ['\x1b', '[', '3', '6', 'm', rank], // 天蓝 in Windows Terminal
            n => panic!("internal card number error: {n}"),
        }
    }
    pub fn print(&self) {
        self.0
            .iter()
            .sorted_by_key(|&(rank, _)| {
                RANK_ORDER
                    .iter()
                    .position(|r| r == rank)
                    .unwrap_or(RANK_ORDER.len())
            })
            .rev()
            .map(|(&rank, &n)| rank.pipe(DeckCards::colored_rank(n)))
            .flatten()
            .chain("\x1b[0m".chars())
            .collect::<String>()
            .echo();
    }
}