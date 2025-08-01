#[derive(Debug, PartialEq, PartialOrd)]
pub enum GameEffect {
    SkipPlayer,
    FlipDirection
}

pub enum PlayerEffect {
    DrawTwo,
    DrawFour
}
