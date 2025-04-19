use std::fmt;

use Face::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

impl Color {
    pub fn iter() -> impl Iterator<Item=Color> {
        [Color::Red, Color::Green, Color::Yellow, Color::Blue].into_iter()
    }

    pub fn as_ansi_code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[37;41m",
            Color::Yellow => "\x1b[30;43m",
            Color::Green => "\x1b[37;42m",
            Color::Blue => "\x1b[37;44m",
        }
    }

    #[inline]
    pub fn blank() -> &'static str {
        "\x1b[30;47m"
    }

    #[inline]
    pub fn ansi_reset_code() -> &'static str {
        "\x1b[0m"
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UncoloredFace {
    PlusFour,
    ChangeColor
}

impl UncoloredFace {
    #[inline]
    pub fn as_regular_face(&self) -> Face {
        self.clone().into()
    }
}

impl fmt::Display for UncoloredFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_regular_face().fmt(f)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
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
    pub fn one_to_plustwo() -> impl Iterator<Item=Face> {
        return [One, Two, Three, Four, Five, Six, Seven,
                Eight, Nine, Skip, FlipDirection, PlusTwo].into_iter()
    }
}

impl From<UncoloredFace> for Face {
    fn from(uncolored: UncoloredFace) -> Self {
        match uncolored {
            UncoloredFace::ChangeColor => Face::ChangeColor,
            UncoloredFace::PlusFour => Face::PlusFour
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let face_str = match self {
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
            Face::ChangeColor => "CHC",
        };
        write!(f, "{:^3}", face_str)
    }
}
