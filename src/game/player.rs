use std::io::{self, Write};

use super::card;
pub use CastResult::*;

pub trait Player {
    fn cast(&mut self, top_card: &card::Card) -> CastResult;
    fn draw(&mut self, deck: &mut card::Deck);
    fn has_cards(&self) -> bool;
}

impl std::fmt::Debug for dyn Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("Player").finish()
    }
}

#[derive(Debug)]
pub struct LocalPlayer {
    hand: card::Deck,
}

impl LocalPlayer {
    pub fn new() -> Self {
        LocalPlayer {
            hand: card::Deck::new(),
        }
    }

    fn get_card_index(&self, candidates: Vec<(usize, &card::Card)>) -> Option<usize> {
        let candidates_string: String = candidates
            .iter()
            .map(|&(i, c)| format!("{}:{}", i, c))
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or(String::from(""));
        println!("Playable cards: {}", candidates_string);
        print!("Card index to cast (or p to pass): ");
        io::stdout().flush().expect("Couldn't flush stdout");

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read from stdin");
            let input = input.trim();

            if input == "p" {
                return None;
            }

            match input.parse::<usize>() {
                Ok(i) if candidates.iter().find(|&&(idx, _)| i == idx).is_some() =>
                    break Some(i),
                _ => {
                    print!("That didn't work out as expected; try again? ");
                    io::stdout().flush().unwrap();
                }
            };
        }
    }
}

impl Player for LocalPlayer {
    fn cast(&mut self, top_card: &card::Card) -> CastResult {
        println!("Your cards: {}", self.hand.cards());
        let candidates = self.hand.get_candidates_for_card(top_card);

        if candidates.len() == 0 {
            println!("You don't have any available cards to cast.");
            return MustDraw;
        }

        match self.get_card_index(candidates) {
            None => Pass,
            Some(index) => {
                let mut card = self.hand.choose(index);
                if card.is_colorless() {
                    card.pick_color();
                }

                Cast(card)
            }
        }
    }

    fn draw(&mut self, deck: &mut card::Deck) {
        self.hand.put(deck.take().expect("deck should never be empty"));
        println!("Drawing a card...");
        self.hand.sort();
    }

    fn has_cards(&self) -> bool {
        !self.hand.empty()
    }
}

pub struct DumbMachinePlayer;
pub struct SmartMachinePlayer;
// maybe NetworkPlayer in the future?

pub enum CastResult {
    Cast(card::Card),
    Pass,
    MustDraw,
}
