# Uno-Rust
CLI Uno card game written in Rust.

***

## Roadmap (updated August 2024)
- [x] Playable base game
  - [x] Against local human players
  - [x] Against machines
    - [x] Dumb ones
    - [x] Smart ones
  - [ ] Against network human players <sub>remote possibility</sub>
- [x] Customizable game rules
- [x] Cards with effects
  - [x] Skip
  - [x] Flip direction
  - [x] +2
  - [x] +4
  - [x] Change color
- [x] Nice cards (see [images](./imgs))
- [ ] Tests (back to square one at this)
  - [ ] Session
  - [ ] Players
    - [ ] Player queues
  - [ ] Cards
    - [ ] Card pickers
    - [ ] Color pickers
  - [ ] Decks
    - [ ] Shuffling
  - [ ] Rules
- [ ] Code-level documentation

## Technical debt
As of this commit, invalid `Card`s can be constructed.

The current implementation requires that a card has a face, and an optional color. Simple, yet
wrong, as many colored cards can be constructed without one. I am to find a good middle point
between simple and correct - already had some ideas, and even if they fix the invalid state issue,
they require some duplication.

Next steps are to come up with a good type-level solution for the cards, and write some tests.
