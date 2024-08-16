use super::{PlayerEffect, SessionEffect};

#[derive(Debug)]
pub enum ApplyEffect {
    UponPlayer(PlayerEffect),
    UponSession(SessionEffect)
}
