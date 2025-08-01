pub mod card_pickers;
pub mod color_pickers;
pub mod local;
pub mod machine;

use crate::cards::{Card, ColoredCard};

pub trait Player {
    fn cast(&mut self, top: &ColoredCard) -> PlayerAction;
    fn react(&mut self, event: &Event) -> Option<PlayerReaction>;
    fn draw(&mut self, card: Card);
    fn has_cards(&self) -> bool;
}

pub type Players = Vec<Box<dyn Player>>;

pub enum PlayerAction {
    Cast(ColoredCard),
    Pass
}

pub enum PlayerReaction {
    Stack(ColoredCard),
    Submit
}
