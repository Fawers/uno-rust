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
