use crate::{
    game_engine::ONE_SECOND_IN_TICKS,
    game_state::{GameItem, ItemAction, ItemTrigger, TriggerType},
};

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
pub struct Boomerang {}
impl Boomerang {
    pub fn new(level: GameItemLevel) -> GameItem {
        let damage = match level {
            GameItemLevel::Bronze => 20,
            GameItemLevel::Silver => 30,
            GameItemLevel::Gold => 40,
            GameItemLevel::Diamond => 50,
        };
        GameItem::new("boomerang", 0, 50, None, vec![ItemAction::Damage(damage)])
    }
}

pub fn boomerang() -> GameItem {
    Boomerang::new(GameItemLevel::Silver)
}

pub fn sunderer() -> GameItem {
    GameItem::new("sunderer", 0, 50, None, vec![ItemAction::Damage(10)])
}

pub struct RestorativeSteamLadle {}
impl RestorativeSteamLadle {
    pub fn new(_: GameItemLevel) -> GameItem {
        GameItem::new(
            "Restorative Steam Ladle",
            0,
            40,
            None,
            vec![ItemAction::Heal(20), ItemAction::Burn(2)],
        )
    }
}

pub struct UwashiwaliBird {}
impl UwashiwaliBird {
    pub fn new(level: GameItemLevel) -> GameItem {
        let healing = match level {
            GameItemLevel::Bronze => 10,
            GameItemLevel::Silver => 20,
            GameItemLevel::Gold => 40,
            GameItemLevel::Diamond => 80,
        };
        GameItem::new(
            "Uwashiwali Bird",
            0,
            50,
            None,
            vec![ItemAction::Heal(healing)],
        )
    }
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

        let mut item = GameItem::new("Weights", 0, 50, None, actions);
        let trigger = ItemTrigger {
            item_id: item.id,
            trigger_type: TriggerType::OnHealing,
            trigger: |action_taken, player, _| {
                let healing_amount = match action_taken {
                    ItemAction::Heal(heal) => heal,
                    _ => 0,
                };

                if healing_amount == 0 {
                    return;
                }

                if healing_amount + player.health <= player.max_health {
                    return;
                }

                let self_item = player.field.iter_mut().find(|item| item.name == "Weights");
                if let Some(item) = self_item {
                    item.ticks_left = item.ticks_left - ONE_SECOND_IN_TICKS;
                }
            },
        };
        item.triggers.push(trigger);
        item
    }
}

pub enum GameItemLevel {
    Bronze,
    Silver,
    Gold,
    Diamond,
}
