pub mod deck;
pub mod types;

use std::{collections::HashMap, hash::Hash};

use deck::Deck;
use types::{Color, Face};

use crate::{effects::apply::ApplyEffect, rules::Rules};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Card(pub Option<Color>, pub Face);

impl Card {
    pub fn stacks_upon(&self, other: &Self) -> bool {
        self.0 == other.0 || self.1 == other.1 || self.0.is_none() || other.0.is_none()
    }

    pub fn effects(&self, rules: &Rules) -> Vec<ApplyEffect> {
        rules.iter()
            .flat_map(|r| r.applies(self))
            .collect()
    }
}

pub fn count_cards_by_key<T, F>(cards: Vec<&Card>, mut key: F) -> Vec<(T, usize)>
where T: Eq + Hash + Clone,
      F: FnMut(&Card) -> T
{
    let mut map = HashMap::new();

    for card in cards {
        map
            .entry(key(card))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|item| -(item.1 as isize));
    v
}

pub fn count_cards_by_color(cards: Vec<&Card>) -> Vec<(Option<Color>, usize)> {
    count_cards_by_key(cards, |c| c.0.clone())
}

pub fn count_cards_by_face(cards: Vec<&Card>) -> Vec<(Face, usize)> {
    count_cards_by_key(cards, |c| c.1.clone())
}

pub fn create_uno_cards() -> Deck {
    let mut d = Deck::new(108);
    let v = d.cards_mut();

    for color in Color::iter() {
        v.push(Card(Some(color), Face::Zero));

        for face in Face::one_to_plustwo() {
            v.push(Card(Some(color), face));
            v.push(Card(Some(color), face));
        }
    }

    for _ in 0..4 {
        v.push(Card(None, Face::PlusFour));
        v.push(Card(None, Face::ChangeColor));
    }

    d
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face = match self.1 {
            Face::Zero => "0",
            Face::One => "1",
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Eight => "8",
            Face::Nine => "9",
            Face::Skip => "SKP",
            Face::FlipDirection => "FLP",
            Face::PlusTwo => "+2",
            Face::PlusFour => "+4",
            Face::ChangeColor => "CHC"
        };
        let color_start = match self.0 {
            None => "\x1b[30;47m",
            Some(Color::Red) => "\x1b[37;41m",
            Some(Color::Yellow) => "\x1b[30;43m",
            Some(Color::Green) => "\x1b[37;42m",
            Some(Color::Blue) => "\x1b[37;44m",
        };

        write!(f, "{}{:^3}{}", color_start, face, "\x1b[0m")
    }
}
