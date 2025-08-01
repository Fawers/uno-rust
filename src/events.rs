use crate::cards::ColoredCard;

pub enum Event<'a> {
    CardCast(&'a ColoredCard)
}
