use crate::cards::deck::Deck;
use crate::cards::types::Face;
use crate::cards::Card;
use crate::effects::{PlayerEffect, SessionEffect, apply::ApplyEffect};
use crate::utils::{dealers::Dealer, players::PlayerQueue, shufflers::Shuffler};

use super::{Session, SessionContext};

const MINIMUM_NECESSARY_FOR_TURN: usize = 3;

pub struct CLISession;

impl<S, D> Session<S, D> for CLISession
where S: Shuffler,
      D: Dealer
{
    fn run(&mut self, mut ctx: SessionContext<S, D>) {
        ctx.deck.shuffle(&mut ctx.shuffler);
        ctx.dealer.deal(&mut ctx.players, ctx.deck.cards_mut());

        let mut discard_pile = Deck::new(16);
        discard_pile.cards_mut().push(ctx.deck.cards_mut().pop().expect("not one card?"));

        let mut queue = PlayerQueue::new(&mut ctx.players);

        loop {
            if ctx.deck.cards().len() < MINIMUM_NECESSARY_FOR_TURN {
                self.redo_deck_from_discard_pile(&mut ctx.deck, &mut discard_pile);
            }

            let p = queue.current();
            let top = *discard_pile.cards().last().expect("where all the cards at");

            match p.cast(top) {
                Some(card) => discard_pile.cards_mut().push(card),
                None => {
                    p.draw(ctx.deck.cards_mut().pop().unwrap());
                    if let Some(card) = p.cast(top) {
                        discard_pile.cards_mut().push(card);
                    }
                    else {
                        queue.step();
                        continue;
                    }
                },
            }

            if !p.has_cards() { break; }

            queue.step();
            let new_top = *discard_pile.cards().last().unwrap();

            new_top.effects(&ctx.rules).into_iter().for_each(|fx| match fx {
                ApplyEffect::UponPlayer(pe) => {
                    let n = if pe == PlayerEffect::Draw2 { 2 } else { 4 };
                    (0..n).for_each(|_| queue.current().draw(ctx.deck.cards_mut().pop().unwrap()))
                },
                ApplyEffect::UponSession(SessionEffect::SkipPlayer) => queue.step(),
                ApplyEffect::UponSession(SessionEffect::FlipDirection) => {
                    queue.flip();
                    queue.step();
                    queue.step(); // ¯\_(ツ)_/¯
                }
            });
        }
    }
}

impl CLISession {
    fn redo_deck_from_discard_pile(&mut self, deck: &mut Deck, pile: &mut Deck) {
        let mut to_reappend = Vec::with_capacity(deck.cards().len());
        let cards = deck.cards_mut();
        let discards = pile.cards_mut();

        to_reappend.append(cards);

        while discards.len() > 1 {
            let mut card = discards.pop().unwrap();
            if let Card(Some(_), Face::ChangeColor | Face::PlusFour) = &card {
                card.0 = None;
            }
            cards.push(card);
        }

        cards.append(&mut to_reappend);
    }
}
