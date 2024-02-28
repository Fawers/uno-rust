use super::card;
use super::player;

pub trait Rules {
    fn allows_stacking(&self) -> bool {
        false
    }

    fn get_effect(&self, card::Card(face, _): &card::Card) -> Option<card::Effect> {
        match face {
            card::Skip => Some(card::Effect::SkipPlayer),
            card::FlipDirection => Some(card::Effect::FlipDirection),
            card::PlusTwo => Some(card::Effect::DrawTwo),
            card::PlusFour => Some(card::Effect::DrawFour),
            _ => None
        }
    }

    fn apply_effect(&self, p: &mut dyn player::Player, fx: &card::Effect) {}
}

pub struct StandardRules;
impl Rules for StandardRules{}
