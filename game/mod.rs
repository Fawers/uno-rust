pub mod card;
pub mod player;
pub mod rules;

#[derive(Debug)]
pub struct Game {
    deck: card::Deck,
    discard_pile: card::Deck,
    players: Vec<Box<dyn player::Player>>,
    current_player: usize,
    flipped: bool,
}

impl Game {
    pub fn new(num_players: usize) -> Self {
        let mut g = Game {
            deck: card::mkdeck(),
            discard_pile: card::Deck::new(),
            players: vec![],
            current_player: 0,
            flipped: false,
        };

        // player creation
        for _ in 0..num_players {
            g.players.push(Box::new(player::LocalPlayer::new()));
        }

        // card distribution
        for _ in 0..7 {
            for p in &mut g.players {
                p.draw(&mut g.deck);
            }
        }

        while let Some(card::Card(_, None)) = g.deck.peek() {
            g.deck.shuffle();
        }

        // draw one card from the deck and
        // cast it onto the discard pile to
        // start the game
        g.discard_pile.put(g.deck.take().unwrap());

        g
    }

    pub fn play(&mut self) {
        println!("Starting a new Uno! game.");
        let mut effect_applied = false;

        loop {
            if !effect_applied && self.apply_effect() {
                effect_applied = true;
                self.set_next_player_index();
                continue;
            }

            let p = &mut self.players[self.current_player];
            let c = self.discard_pile.peek().unwrap();
            println!("\nCard on top: {}", c);
            println!("Player {}'s turn", self.current_player);

            match p.cast(c) {
                player::Cast(card) => {
                    self.discard_pile.put(card);
                    effect_applied = false;
                },
                player::Pass => println!("Player {} passed.", self.current_player),
                player::MustDraw => {
                    p.draw(&mut self.deck);
                    match p.cast(c) {
                        player::Cast(card) => {
                            self.discard_pile.put(card);
                            effect_applied = false;
                        },
                        _ => println!("Player {} passed.", self.current_player)
                    }
                }
            }

            if !p.has_cards() {
                println!("Player {} won the game!", self.current_player);
                break;
            }

            if let Some(card::Card(card::FlipDirection, _)) = self.deck.peek() {
                self.flipped = !self.flipped;
                effect_applied = true;
            }

            self.set_next_player_index();
            self.check_for_empty_deck();
        }
    }

    fn check_for_empty_deck(&mut self) {
        if self.deck.empty() {
            let last_card = self.discard_pile.take().unwrap();

            self.discard_pile.shuffle();
            while !self.discard_pile.empty() {
                self.deck.put(self.discard_pile.take().unwrap());
            }

            self.discard_pile.put(last_card);
        }
    }

    fn set_next_player_index(&mut self) {
        self.current_player = match (self.flipped, self.current_player) {
            (true, 0) => self.players.len() - 1,
            (true, n) => n - 1,
            (false, n) => (n + 1) % self.players.len()
        };
    }

    fn apply_effect(&mut self) -> bool {
        let draw_count: u32;

        match self.discard_pile.peek() {
            None => false,
            Some(card::Card(face, _)) => {
                match face {
                    card::PlusTwo => draw_count = 2,
                    card::PlusFour => draw_count = 4,
                    card::Skip => return true,
                    card::FlipDirection => {
                        self.flipped = !self.flipped;
                        return true;
                    }
                    _ => return false,
                }

                for _ in 0..draw_count {
                    self.players[self.current_player].draw(&mut self.deck);
                    self.check_for_empty_deck();
                }

                true
            }
        }
    }
}
