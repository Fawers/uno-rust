use crate::utils::shufflers::Shuffler;

use super::Card;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new(cap: usize) -> Self {
        Deck { cards: Vec::with_capacity(cap) }
    }

    pub fn cards(&self) -> Vec<&Card> {
        self.cards.iter().collect()
    }

    pub fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn get_stackable_cards(&self, card: &Card) -> Vec<(usize, &Card)> {
        self.cards.iter()
            .enumerate()
            .filter(|&c| c.1.stacks_upon(card))
            // .map(|(index, _)| index)
            .collect()
    }

    pub fn shuffle<S: Shuffler>(&mut self, shuffler: &mut S) {
        shuffler.shuffle(&mut self.cards)
    }
}
