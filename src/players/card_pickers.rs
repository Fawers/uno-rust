use crate::cards::{self, deck::Deck, Card};

pub trait CardPicker {
    fn pick(&mut self, hand: &mut Deck, top: &Card) -> Option<Card>;
}

pub fn new_dumb_picker() -> impl CardPicker {
    DumbPicker
}

pub fn new_smart_picker() -> impl CardPicker {
    SmartPicker
}

struct DumbPicker;

impl CardPicker for DumbPicker {
    fn pick(&mut self, hand: &mut Deck, top: &Card) -> Option<Card> {
        let candidates = hand.get_stackable_cards(top);
        match candidates.get(0) {
            Some(&(index, _)) => Some(hand.cards_mut().remove(index)),
            None => None
        }
    }
}

struct SmartPicker;

impl CardPicker for SmartPicker {
    fn pick(&mut self, hand: &mut Deck, top: &Card) -> Option<Card> {
        let candidates = hand.get_stackable_cards(top);
        let cards = || candidates.iter().map(|&c| c.1).collect::<Vec<_>>();
        let by_color = cards::count_cards_by_color(cards());
        let by_face = cards::count_cards_by_face(cards());

        let index_to_remove = match (by_color.first(), by_face.first()) {
            (Some(color), Some(face)) => {
                if color.1 > face.1 {
                    candidates.iter().find(|&c| c.1.0 == color.0).map(|c| c.0)
                }
                else {
                    candidates.iter().find(|&c| c.1.1 == face.0).map(|c| c.0)
                }
            },
            (Some(&(_, index)), None) => Some(index),
            (None, Some(&(_, index))) => Some(index),
            _ => None
        };

        index_to_remove.map(|i| hand.cards_mut().remove(i))
    }
}
