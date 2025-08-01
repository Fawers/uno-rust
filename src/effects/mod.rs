pub mod apply;

#[derive(Debug, PartialEq)]
pub enum CardEffect {
    Draw2,
    Draw4,
    SkipPlayer,
    FlipDirection
}
