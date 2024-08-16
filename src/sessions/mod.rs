use crate::{cards::deck::Deck, players::Players, rules::Rules, utils::{dealers::Dealer, shufflers::Shuffler}};

pub mod local;

pub trait Session<S, D>
where S: Shuffler,
      D: Dealer
{
    fn run(&mut self, ctx: SessionContext<S, D>);
}

pub struct SessionContext<S, D> {
    pub rules: Rules,
    pub players: Players,
    pub deck: Deck,
    pub shuffler: S,
    pub dealer: D
}
