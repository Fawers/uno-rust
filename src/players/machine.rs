use crate::{cards::{deck::Deck, Card}, utils::loggers::Logger};

use super::{card_pickers::CardPicker, color_pickers::ColorPicker, Player};

pub struct MachinePlayer<L, Cap, Cop> {
    id: String,
    hand: Deck,
    logger: L,
    card_picker: Cap,
    color_picker: Cop
}

impl<L, Cap, Cop> MachinePlayer<L, Cap, Cop> {
    pub fn new(identifier: String, hand: Deck, logger: L, cap: Cap, cop: Cop) -> Self {
        Self {id: identifier, hand, logger, card_picker: cap, color_picker: cop}
    }
}

impl<L, Cap, Cop> Player for MachinePlayer<L, Cap, Cop>
where L: Logger,
      Cap: CardPicker,
      Cop: ColorPicker
{
    fn cast(&mut self, top: &Card) -> Option<Card> {
        let mut card = self.card_picker.pick(&mut self.hand, top);

        match card {
            Some(ref mut c) => {
                if c.0.is_none() {
                    c.0 = Some(self.color_picker.pick(&mut self.hand))
                }
                self.logger.log(format!("{} casts {c}.", self.id));
                card
            },
            None => {
                self.logger.log(format!("{} doesn't have cards to cast.", self.id));
                None
            }
        }
    }

    fn draw(&mut self, card: crate::cards::Card) {
        self.logger.log(format!("{} drew a card", self.id));
        self.hand.cards_mut().push(card);
    }

    fn has_cards(&self) -> bool {
        self.hand.cards().len() > 0
    }
}
