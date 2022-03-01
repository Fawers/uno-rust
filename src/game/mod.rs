pub mod card;
pub mod player;

#[derive(Debug)]
pub struct Game;

impl Game {
    pub fn play(&mut self) {
        println!("Playing game");
    }
}
