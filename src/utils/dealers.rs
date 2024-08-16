use crate::{cards::Card, players::Players};

pub trait Dealer {
    fn deal(&mut self, players: &mut Players, cards: &mut Vec<Card>);
}

pub fn new() -> impl Dealer {
    StandardDealer
}

struct StandardDealer;

impl Dealer for StandardDealer {
    fn deal(&mut self, players: &mut Players, cards: &mut Vec<Card>) {
        for _ in 0..7 {
            for p in players.iter_mut() {
                p.draw(cards.pop().expect("we're supposed to have enough cards at this point"));
            }
        }
    }
}
