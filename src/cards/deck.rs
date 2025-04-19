use crate::utils::shufflers::Shuffler;

use super::{types::{Color, Face, UncoloredFace}, Card, ColoredCard};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub(crate) fn new(cap: usize) -> Self {
        Deck { cards: Vec::with_capacity(cap) }
    }

    pub fn create_uno_deck() -> Self {
        let mut d = Self::new(108);
        let v = d.cards_mut();

        for color in Color::iter() {
            v.push(Card::colored(Face::Zero, color));

            for face in Face::one_to_plustwo() {
                v.push(Card::colored(face, color));
                v.push(Card::colored(face, color));
            }
        }

        for _ in 0..4 {
            v.push(Card::uncolored(UncoloredFace::PlusFour));
            v.push(Card::uncolored(UncoloredFace::ChangeColor));
        }

        d
    }

    pub fn cards(&self) -> Vec<&Card> {
        self.cards.iter().collect()
    }

    pub fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn get_stackable_cards(&self, card: &ColoredCard) -> Vec<(usize, &Card)> {
        self.cards.iter()
            .enumerate()
            .filter(|&c| c.1.can_stack_upon(card))
            // .map(|(index, _)| index)
            .collect()
    }

    pub fn shuffle<S: Shuffler>(&mut self, shuffler: &mut S) {
        shuffler.shuffle(&mut self.cards)
    }
}
