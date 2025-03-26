use crate::{cards, game_state::PlayerState};

pub fn example_player1() -> PlayerState {
    PlayerState::new(
        370,
        vec![
            cards::boomerang(),
            cards::sunderer(),
            cards::improvised_bludgeon(),
        ],
        "P1".to_string(),
    )
}
