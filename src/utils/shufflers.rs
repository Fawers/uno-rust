use crate::cards::Card;
use rand::seq::SliceRandom;

pub trait Shuffler {
    fn shuffle(&mut self, cards: &mut [Card]);
}

pub fn new() -> impl Shuffler {
    StandardShuffler
}

struct StandardShuffler;

impl Shuffler for StandardShuffler {
    fn shuffle(&mut self, cards: &mut [Card]) {
        let mut r = rand::thread_rng();
        cards.shuffle(&mut r);
    }
}
