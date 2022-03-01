use rand;
use rand::seq::SliceRandom;

pub use Face::*;
pub use Color::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Face {
    Zero, One, Two, Three, Four,
    Five, Six, Seven, Eight, Nine,
    Skip, FlipDirection, PlusTwo, PlusFour, ChangeColor
}

impl Face {
    fn slice() -> &'static [Face] {
        &[
            Zero, One, Two, Three, Four,
            Five, Six, Seven, Eight, Nine,
            Skip, FlipDirection, PlusTwo, PlusFour, ChangeColor
        ]
    }

    fn one_to_plustwo() -> &'static [Face] {
        let s = Self::slice();
        &s[1..s.len()-2]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue
}

impl Color {
    fn slice() -> &'static [Color] {
        &[Red, Yellow, Green, Blue]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(pub Face, pub Option<Color>);

impl Card {
    pub fn can_be_placed_onto(&self, card: &Card) -> bool {
        match (self, card) {
            (Card(PlusFour | ChangeColor, _), _) => true,
            (Card(lface, _), Card(rface, _)) if lface == rface => true,
            (Card(_, Some(lcolor)), Card(_, Some(rcolor))) if lcolor == rcolor => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Deck(Vec<Card>, rand::prelude::ThreadRng);

impl Deck {
    pub fn new() -> Self {
        Self(vec![], rand::thread_rng())
    }

    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut self.1);
    }
}

pub fn mkdeck() -> Deck {
    let mut deck = Deck::new();

    for &color in Color::slice() {
        deck.0.push(Card(Zero, Some(color)));
    }

    for &face in Face::one_to_plustwo() {
        for &color in Color::slice() {
            deck.0.push(Card(face, Some(color)));
            deck.0.push(Card(face, Some(color)));
        }
    }

    for _ in 0..4 {
        deck.0.push(Card(PlusFour, None));
        deck.0.push(Card(ChangeColor, None))
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
            .filter(|Card(face, color)| {
                color == &None &&
                [PlusFour, ChangeColor].contains(face)})
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
