use uno::cards::{create_uno_cards, deck::Deck};
use uno::players::{card_pickers, color_pickers};
use uno::players::{local::LocalHumanPlayer, machine::MachinePlayer};
use uno::rules;
use uno::sessions::{Session, SessionContext, local::CLISession};
use uno::utils::{dealers, shufflers, loggers::stdout_logger};

fn main() {
    let context = SessionContext {
        rules: vec![
            Box::new(rules::draw_2_rule()),
            Box::new(rules::draw_4_rule()),
            Box::new(rules::skip_player_rule()),
            Box::new(rules::flip_direction_rule())],
        players: vec![
            Box::new(LocalHumanPlayer::new(
                Deck::new(8),
                stdout_logger())),
            Box::new(MachinePlayer::new(
                "Bot 1 (smart)".into(),
                Deck::new(8),
                stdout_logger(),
                card_pickers::new_smart_picker(),
                color_pickers::new_smart_picker())),
            Box::new(MachinePlayer::new(
                "Bot 2 (dumb af)".into(),
                Deck::new(8),
                stdout_logger(),
                card_pickers::new_dumb_picker(),
                color_pickers::new_dumb_picker()))],
        deck: create_uno_cards(),
        shuffler: shufflers::new(),
        dealer: dealers::new()
    };

    let mut sesh = CLISession;
    sesh.run(context);
}
