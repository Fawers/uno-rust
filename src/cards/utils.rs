use std::collections::HashMap;
use std::hash::Hash;

use super::types::{Color, Face};
use super::{Card, ColoredCard, UncoloredCard};

fn count_cards_by_key<T, F>(cards: Vec<&Card>, mut key: F) -> Vec<(T, usize)>
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
    count_cards_by_key(cards, |c| match c {
        Card::Uncolored(_) => None,
        Card::Colored(ColoredCard(_, color)) => Some(color.clone())
    })
}

pub fn count_cards_by_face(cards: Vec<&Card>) -> Vec<(Face, usize)> {
    count_cards_by_key(cards, |c| match c {
        Card::Uncolored(UncoloredCard(face)) => face.as_regular_face(),
        Card::Colored(ColoredCard(face, _)) => face.clone()
    })
}
