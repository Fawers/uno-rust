use rand;
use rand::seq::SliceRandom;

use super::*;

#[derive(Debug)]
pub struct Deck(Vec<Card>, rand::prelude::ThreadRng);

impl Deck {
    pub fn new() -> Self {
        Self(vec![], rand::thread_rng())
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut self.1);
    }

    pub fn sort(&mut self) {
        self.0.sort_unstable_by_key(|&Card(face, color)| (color, face));
    }

    pub fn empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn put(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn take(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn choose(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    pub fn peek(&self) -> Option<&Card> {
        self.0.last()
    }

    pub fn cards(&self) -> String {
        self.0.iter()
            .map(ToString::to_string)
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or_else(String::new)
    }

    pub fn get_candidates_for_card(&self, card: &Card) -> Vec<(usize, &Card)> {
        self.0
            .iter()
            .enumerate()
            .filter(|&(_, c)| c.can_be_placed_onto(card))
            .collect()
    }
}

pub fn mkdeck() -> Deck {
    let mut deck = Deck::new();

    for &color in Color::slice() {
        deck.put(Card(Zero, Some(color)));
    }

    for &face in Face::one_to_plustwo() {
        for &color in Color::slice() {
            deck.put(Card(face, Some(color)));
            deck.put(Card(face, Some(color)));
        }
    }

    for _ in 0..4 {
        deck.put(Card(PlusFour, None));
        deck.put(Card(ChangeColor, None))
    }

    deck.shuffle();
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_has_108_cards() {
        assert_eq!(mkdeck().0.len(), 108);
    }

    #[test]
    fn deck_has_4_zeroes() {
        let num_cards = mkdeck().0
            .iter()
            .filter(|Card(face, _)| face == &Zero)
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(num_cards, 4);
    }

    #[test]
    fn deck_has_8_skip_cards() {
        let num_cards = mkdeck().0
            .iter()
            .filter(|Card(face, _)| face == &Skip)
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(num_cards, 8);
    }

    #[test]
    fn deck_has_25_red_cards() {
        let num_cards = mkdeck().0
            .iter()
            .filter(|Card(_, color)| color == &Some(Red))
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(num_cards, 25);
    }

    #[test]
    fn deck_has_2_blue_flipdir_cards() {
        let num_cards = mkdeck().0
            .iter()
            .filter(|Card(face, color)| face == &FlipDirection && color == &Some(Blue))
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(num_cards, 2);
    }

    #[test]
    fn deck_has_8_plusfour_and_changecolor_cards() {
        let num_cards = mkdeck().0
            .iter()
            .filter(|Card(face, color)| color == &None && [PlusFour, ChangeColor].contains(face))
            .collect::<Vec<&Card>>()
            .len();
        assert_eq!(num_cards, 8);
    }

    #[test]
    fn no_two_decks_are_equal_in_order() {
        let (mut d1, mut d2) = (mkdeck(), mkdeck());
        assert_ne!(d1.0, d2.0);
        d1.0.sort_unstable();
        d2.0.sort_unstable();
        assert_eq!(d1.0, d2.0);
    }
}
