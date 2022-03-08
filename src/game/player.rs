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

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read from stdin");
        input
    }

    fn get_card_index<F>(&self, candidates: Vec<(usize, &card::Card)>, read: F) -> Option<usize>
    where F: Fn() -> String {
        let candidates_string: String = candidates
            .iter()
            .map(|&(i, c)| format!("{}:{}", i, c))
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or_else(String::new);
        println!("Playable cards: {}", candidates_string);
        print!("Card index to cast (or p to pass): ");
        io::stdout().flush().expect("Couldn't flush stdout");

        loop {
            let input = read();
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

        match self.get_card_index(candidates, Self::get_input) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_with_empty_deck_has_no_cards() {
        let p = LocalPlayer::new();
        assert!(!p.has_cards());
    }

    #[test]
    fn player_with_nonempty_deck_has_cards() {
        let mut p = LocalPlayer::new();
        p.hand.put(card::Card(card::Zero, Some(card::Red)));

        assert!(p.has_cards());
    }

    #[test]
    fn drawing_a_card_adds_it_to_hand() {
        let mut p = LocalPlayer::new();
        let mut d = card::Deck::new();

        d.put(card::Card(card::Zero, Some(card::Red)));
        p.draw(&mut d);

        let c = p.hand.peek();
        assert_eq!(c, Some(&card::Card(card::Zero, Some(card::Red))));
        assert_eq!(d.peek(), None);
    }

    #[test]
    fn get_card_index_returns_none_for_input_p() {
        let mut p = LocalPlayer::new();
        let card = card::Card(card::Zero, Some(card::Blue));

        p.hand.put(card::Card(card::Zero, Some(card::Red)));

        let output = p.get_card_index(
            p.hand.get_candidates_for_card(&card),
            || String::from("p"));

        assert_eq!(output, None);
    }

    #[test]
    fn get_card_index_returns_some0_for_input_0() {
        let mut p = LocalPlayer::new();
        let card = card::Card(card::Zero, Some(card::Blue));

        p.hand.put(card::Card(card::Zero, Some(card::Red)));

        let output = p.get_card_index(
            p.hand.get_candidates_for_card(&card),
            || String::from("0"));

        assert_eq!(output, Some(0));
    }
}
