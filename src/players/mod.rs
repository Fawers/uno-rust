pub mod card_pickers;
pub mod color_pickers;
pub mod local;
pub mod machine;

use crate::cards::Card;

pub trait Player {
    fn cast(&mut self, top: &Card) -> Option<Card>;
    fn draw(&mut self, card: Card);
    fn has_cards(&self) -> bool;
}

pub type Players = Vec<Box<dyn Player>>;
