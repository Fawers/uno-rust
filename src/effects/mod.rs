pub mod apply;

#[derive(Debug, PartialEq)]
pub enum PlayerEffect {
    Draw2,
    Draw4
}

#[derive(Debug, PartialEq)]
pub enum SessionEffect {
    SkipPlayer,
    FlipDirection
}
