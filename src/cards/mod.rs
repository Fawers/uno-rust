pub mod deck;
pub mod types;
pub mod utils;

use types::{Color, Face, UncoloredFace};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UncoloredCard(pub UncoloredFace);

impl UncoloredCard {
    pub fn into_colored(self, color: Color) -> ColoredCard {
        ColoredCard(self.0.into(), color)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColoredCard(pub Face, pub Color);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Card {
    Uncolored(UncoloredCard),
    Colored(ColoredCard)
}

impl Card {
    pub fn can_stack_upon(&self, card_on_top: &ColoredCard) -> bool {
        let ColoredCard(face_on_top, color_on_top) = card_on_top;
        match self {
            Card::Uncolored(_) => true,
            Card::Colored(ColoredCard(face, color)) => {
                face == face_on_top || color == color_on_top
            }
        }
    }

    #[inline]
    pub fn uncolored(face: UncoloredFace) -> Self {
        Self::Uncolored(UncoloredCard(face))
    }

    #[inline]
    pub fn colored(face: Face, color: Color) -> Self {
        Self::Colored(ColoredCard(face, color))
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (face, color_ansi) = match self {
            Card::Uncolored(UncoloredCard(face)) => (face.to_string(), Color::blank()),
            Card::Colored(ColoredCard(face, color)) => (face.to_string(), color.as_ansi_code())
        };

        write!(f, "{}{}{}", color_ansi, face, Color::ansi_reset_code())
    }
}
