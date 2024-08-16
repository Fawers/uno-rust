use crate::cards::{Card, types::Face};
use crate::effects::{apply::ApplyEffect, PlayerEffect, SessionEffect};

pub trait Rule {
    fn applies(&self, card: &Card) -> Vec<ApplyEffect>;
}

pub type Rules = Vec<Box<dyn Rule>>;

pub struct FaceRule<F> {
    face: Face,
    vec_factory: F
}

impl<F> Rule for FaceRule<F>
where F: Fn() -> Vec<ApplyEffect>
{
    fn applies(&self, card: &Card) -> Vec<ApplyEffect> {
        match card.1 {
            face if face == self.face => (self.vec_factory)(),
            _ => vec![]
        }
    }
}

pub fn draw_2_rule() -> impl Rule {
    FaceRule {
        face: Face::PlusTwo,
        vec_factory: || vec![ApplyEffect::UponPlayer(PlayerEffect::Draw2),
                             ApplyEffect::UponSession(SessionEffect::SkipPlayer)]
    }
}

pub fn draw_4_rule() -> impl Rule {
    FaceRule {
        face: Face::PlusFour,
        vec_factory: || vec![ApplyEffect::UponPlayer(PlayerEffect::Draw4),
                             ApplyEffect::UponSession(SessionEffect::SkipPlayer)]
    }
}

pub fn skip_player_rule() -> impl Rule {
    FaceRule {
        face: Face::Skip,
        vec_factory: || vec![ApplyEffect::UponSession(SessionEffect::SkipPlayer)]
    }
}

pub fn flip_direction_rule() -> impl Rule {
    FaceRule {
        face: Face::FlipDirection,
        vec_factory: || vec![ApplyEffect::UponSession(SessionEffect::FlipDirection)]
    }
}
