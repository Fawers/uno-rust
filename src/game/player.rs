use super::card;

pub trait Player {
    fn cast(&mut self) -> card::Card;
    fn draw(&mut self, deck: &mut card::Deck) -> &card::Card;
}

pub struct LocalPlayer;
pub struct DumbMachinePlayer;
pub struct SmartMachinePlayer;

// maybe NetworkPlayer in the future?
