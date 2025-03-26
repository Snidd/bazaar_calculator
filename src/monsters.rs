use crate::game_state::PlayerState;

pub fn rogue_scrapper() -> PlayerState {
    PlayerState {
        name: "RS".to_string(),
        health: 450,
        max_health: 450,
        shield: 0,
        poison: 0,
        burn: 0,
        field: vec![
            crate::cards::gearnola_bar(),
            crate::cards::junkyard_club(),
            crate::cards::junkyard_repairbot(),
            crate::cards::barbed_wire(),
        ],
    }
}
