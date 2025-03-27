use crate::game_state::PlayerState;

pub fn rogue_scrapper() -> PlayerState {
    PlayerState::new(
        450,
        vec![
            crate::cards::gearnola_bar(),
            crate::cards::junkyard_club(),
            crate::cards::junkyard_repairbot(),
            crate::cards::barbed_wire(),
        ],
        "Rogue Scrapper".to_string(),
    )
}
