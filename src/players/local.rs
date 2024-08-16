use std::io::{self, Write};

use crate::{cards::{deck::Deck, types::Color, Card}, utils::loggers::Logger};

use super::{card_pickers::CardPicker, color_pickers::ColorPicker, Player};

pub struct LocalHumanPlayer<L> {
    hand: Deck,
    logger: L,
}

impl<L> LocalHumanPlayer<L> {
    pub fn new(hand: Deck, logger: L) -> Self {
        Self {hand, logger}
    }
}

impl<L> Player for LocalHumanPlayer<L>
where L: Logger,
{
    fn cast(&mut self, top: &Card) -> Option<Card> {
        let mut card = LocalCardPicker(&mut self.logger).pick(&mut self.hand, top);

        match card {
            Some(ref mut c) if c.0.is_none() => {
                c.0 = Some(LocalColorPicker.pick(&mut self.hand));
            },
            _ => ()
        }

        self.logger.log(card.as_ref()
            .map_or_else(|| "Passing".into(),
                         |c| format!("Casting {c}")));
        card
    }

    fn draw(&mut self, card: Card) {
        self.logger.log(format!("Drawing {card}"));
        self.hand.cards_mut().push(card);
        self.hand.cards_mut().sort_unstable();
    }

    fn has_cards(&self) -> bool {
        self.hand.cards().len() > 0
    }
}

struct LocalCardPicker<'a, L>(&'a mut L);

impl<'a, L: Logger> CardPicker for LocalCardPicker<'a, L> {
    fn pick(&mut self, hand: &mut Deck, top: &Card) -> Option<Card> {
        let hand_str = hand.cards().iter()
            .map(|&c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let candidates = hand.get_stackable_cards(top);
        let castables = candidates.iter()
            .map(|&(i, c)| format!("({i}) {c}"))
            .collect::<Vec<_>>()
            .join(", ");
        self.0.log(format!("\nCard on top: {top}"));
        self.0.log(format!("Hand: {hand_str}"));
        self.0.log(format!("Castable cards: {castables}"));

        let index: usize = loop {
            let mut buf = String::new();
            print!("Input card index, or \x1b[1mpass\x1b[0m to pass.\n> ");
            io::stdout().flush().expect("stdout be clogged?");
            io::stdin().read_line(&mut buf).expect("whachu mean you can't read from stdin");

            if buf.starts_with("pass") { return None }

            match buf.trim().parse::<usize>() {
                Ok(i) if candidates.iter().find(|&&(i_, _)| i == i_).is_some() => break i,
                Ok(_) => self.0.log("Cannot cast this card.".into()),
                Err(_) => {
                    self.0.log("Not a number.".into());
                    continue;
                }
            }
        };

        Some(hand.cards_mut().remove(index))
    }
}

struct LocalColorPicker;

impl ColorPicker for LocalColorPicker {
    fn pick(&mut self, _: &mut Deck) -> Color {
        let mut buf = String::new();
        let colors = vec!["red", "green", "yellow", "blue"];

        while !colors.contains(&buf.trim()) {
            buf.clear();
            print!("Input a color: {}, {}, {}, or {}.\n> ",
                   "\x1b[31mred\x1b[0m",
                   "\x1b[32mgreen\x1b[0m",
                   "\x1b[33myellow\x1b[0m",
                   "\x1b[34mblue\x1b[0m");
            io::stdout().flush().expect("stdout be clogged?");
            io::stdin().read_line(&mut buf).expect("whachu mean you can't read from stdin");
        }

        for (name, color) in colors.into_iter().zip(Color::iter()) {
            if buf.trim().starts_with(name) { return color; }
        }

        panic!("unknown color {buf:?}");
    }
}
