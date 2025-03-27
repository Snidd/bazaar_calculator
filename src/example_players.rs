use crate::{
    cards::{self, *},
    game_state::PlayerState,
};

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

pub fn example_kripp1() -> PlayerState {
    let fields = vec![
        RestorativeSteamLadle::new(GameItemLevel::Bronze),
        UwashiwaliBird::new(GameItemLevel::Bronze),
        Boomerang::new(GameItemLevel::Bronze),
        Weights::new(GameItemLevel::Silver),
    ];
    PlayerState::new(370, fields, "Kripp1".to_string())
}
