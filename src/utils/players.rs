use crate::players::{Player, Players};

use Direction::*;

pub struct PlayerQueue<'a> {
    players: &'a mut Players,
    current: usize,
    direction: Direction
}

impl<'a> PlayerQueue<'a> {
    pub fn new(players: &'a mut Players) -> Self {
        Self { players, current: 0, direction: Right}
    }

    pub fn flip(&mut self) {
        self.direction = match self.direction {
            Left => Right,
            Right => Left
        }
    }

    pub fn step(&mut self) {
        let len = self.players.len();
        self.current = match (&self.direction, self.current) {
            (Left, 0) => len-1,
            (Left, cur) => cur-1,
            (Right, last) if last == len-1 => 0,
            (Right, cur) => cur+1
        };
    }

    pub fn current(&mut self) -> &mut dyn Player{
        self.players[self.current].as_mut()
    }
}

enum Direction {
    Left,
    Right
}
