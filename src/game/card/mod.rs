pub mod deck;

use std::io;

pub use Color::*;
pub use Face::*;
pub use deck::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Face {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Skip,
    FlipDirection,
    PlusTwo,
    PlusFour,
    ChangeColor,
}

impl Face {
    fn slice() -> &'static [Face] {
        &[
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Skip,
            FlipDirection,
            PlusTwo,
            PlusFour,
            ChangeColor,
        ]
    }

    fn one_to_plustwo() -> &'static [Face] {
        let s = Self::slice();
        &s[1..s.len() - 2]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

impl Color {
    fn slice() -> &'static [Color] {
        &[Red, Yellow, Green, Blue]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(pub Face, pub Option<Color>);

impl Card {
    pub fn can_be_placed_onto(&self, card: &Card) -> bool {
        match (self, card) {
            (Card(PlusFour | ChangeColor, _), _) => true,
            (Card(lface, _), Card(rface, _)) if lface == rface => true,
            (Card(_, Some(lcolor)), Card(_, Some(rcolor))) if lcolor == rcolor => true,
            _ => false,
        }
    }

    pub fn is_colorless(&self) -> bool {
        self.1.is_none()
    }

    pub fn pick_color(&mut self) {
        if self.0 != PlusFour && self.0 != ChangeColor {
            panic!("Can only pick colors for +4 or ChangeColor cards!");
        }

        if self.1.is_some() {
            panic!("Card already has a color!");
        }

        let color = loop {
            let mut input = String::new();
            println!("Pick a color: {}, {}, {}, {}",
                "\x1b[31mred\x1b[0m",
                "\x1b[33myellow\x1b[0m",
                "\x1b[32mgreen\x1b[0m",
                "\x1b[34mblue\x1b[0m");

            io::stdin().read_line(&mut input).unwrap();
            break match input.to_lowercase().trim() {
                "red" => Red,
                "yellow" => Yellow,
                "green" => Green,
                "blue" => Blue,
                _ => continue,
            };
        };
        self.1 = Some(color);
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face = match self.0 {
            Zero => "0",
            One => "1",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Skip => "∅",
            FlipDirection => "🗘",
            PlusTwo => "+2",
            PlusFour => "+4",
            ChangeColor => "⊕"
        };
        let color_start = match self.1 {
            None => "\x1b[30;47m",
            Some(Red) => "\x1b[37;41m",
            Some(Yellow) => "\x1b[30;43m",
            Some(Green) => "\x1b[37;42m",
            Some(Blue) => "\x1b[37;44m",
        };

        write!(f, "{}{:^3}{}", color_start, face, "\x1b[0m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_card_can_be_placed_on_red_card() {
        let c1 = Card(One, Some(Red));
        let c2 = Card(Skip, Some(Red));
        assert!(c1.can_be_placed_onto(&c2));
    }

    #[test]
    fn yellow_card_cannot_be_placed_on_blue_card_of_different_faces() {
        let c1 = Card(Three, Some(Yellow));
        let c2 = Card(Five, Some(Blue));
        assert!(!c1.can_be_placed_onto(&c2));
    }

    #[test]
    fn can_place_card_of_same_face_onto_another() {
        let c1 = Card(PlusTwo, Some(Green));
        let c2 = Card(PlusTwo, Some(Red));
        assert!(c1.can_be_placed_onto(&c2));
    }

    #[test]
    fn cannot_place_card_of_different_face_onto_another() {
        let c1 = Card(PlusTwo, Some(Green));
        let c2 = Card(FlipDirection, Some(Red));
        assert!(!c1.can_be_placed_onto(&c2));
    }

    #[test]
    #[should_panic]
    fn picking_color_for_non_special_card_panics() {
        Card(Zero, None).pick_color();
    }

    #[test]
    #[should_panic]
    fn picking_color_for_colored_special_card_panics() {
        Card(PlusFour, Some(Red)).pick_color();
    }
}
