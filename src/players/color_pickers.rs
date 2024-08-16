use crate::cards::{count_cards_by_color, deck::Deck, types::Color};

pub trait ColorPicker {
    fn pick(&mut self, hand: &mut Deck) -> Color;
}

pub fn new_dumb_picker() -> impl ColorPicker {
    DumbPicker
}

pub fn new_smart_picker() -> impl ColorPicker {
    SmartPicker {
        default_factory: || Color::Red
    }
}

struct DumbPicker;

impl ColorPicker for DumbPicker {
    fn pick(&mut self, hand: &mut Deck) -> Color {
        hand.cards().iter()
            .find(|&&c| c.0.is_some())
            .map_or_else(|| Color::Red,
                         |&c| c.0.unwrap())
    }
}

struct SmartPicker<F> {
    default_factory: F
}

impl<F: FnMut() -> Color> ColorPicker for SmartPicker<F> {
    fn pick(&mut self, hand: &mut Deck) -> Color {
        count_cards_by_color(hand.cards()).into_iter()
            .find(|(c, _)| c.is_some())
            .map_or_else(|| (self.default_factory)(),
                         |(c, _)| c.unwrap())
    }
}
