use crate::game_state::{GameItem, ItemAction};

pub fn junkyard_club() -> GameItem {
    GameItem::new("Junkyard Club", 0, 110, None, vec![ItemAction::Damage(60)])
}

pub fn junkyard_repairbot() -> GameItem {
    GameItem::new(
        "Junkyard Repairbot",
        0,
        80,
        None,
        vec![ItemAction::Heal(60)],
    )
}

pub fn barbed_wire() -> GameItem {
    GameItem::new("barbed_wire", 0, 50, None, vec![ItemAction::Damage(10)])
}

pub fn gearnola_bar() -> GameItem {
    GameItem::new("gearnola_bar", 0, 50, Some(2), vec![ItemAction::Shield(30)])
}

pub fn improvised_bludgeon() -> GameItem {
    GameItem::new(
        "improvised_bludgeon",
        0,
        80,
        None,
        vec![ItemAction::Damage(40)],
    )
}
//ItemAction::Slow(2, 4)

pub fn boomerang() -> GameItem {
    GameItem::new("boomerang", 0, 50, None, vec![ItemAction::Damage(30)])
}

pub fn sunderer() -> GameItem {
    GameItem::new("sunderer", 0, 50, None, vec![ItemAction::Damage(10)])
}

pub struct Weights {}

impl Weights {
    pub fn new(level: GameItemLevel) -> GameItem {
        let actions = match level {
            GameItemLevel::Bronze => vec![
                ItemAction::IncreaseAllDamage(10),
                ItemAction::IncreaseAllHealing(10),
            ],
            GameItemLevel::Silver => vec![
                ItemAction::IncreaseAllDamage(15),
                ItemAction::IncreaseAllHealing(15),
            ],
            GameItemLevel::Gold => vec![
                ItemAction::IncreaseAllDamage(20),
                ItemAction::IncreaseAllHealing(20),
            ],
            GameItemLevel::Diamond => vec![
                ItemAction::IncreaseAllDamage(25),
                ItemAction::IncreaseAllHealing(25),
            ],
        };
        GameItem::new("Weights", 0, 50, None, actions)
    }
}

pub enum GameItemLevel {
    Bronze,
    Silver,
    Gold,
    Diamond,
}
