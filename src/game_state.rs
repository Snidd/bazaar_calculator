use itertools::Itertools;
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct GameState {
    pub player1: PlayerState,
    pub player2: PlayerState,
    pub current_tick: i32,
    pub sandstorm_damage: i32,
}

#[derive(Clone, Debug)]
pub struct GameAction {
    pub action_tick: f32,
    pub item: GameItem,
}

impl GameState {
    pub fn new(player1: PlayerState, player2: PlayerState) -> GameState {
        GameState {
            player1,
            player2,
            current_tick: 0,
            sandstorm_damage: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub shield: i32,
    pub poison: i32,
    pub burn: i32,
    pub field: Vec<GameItem>,
    pub triggers: Vec<ItemTrigger>,
}

impl PlayerState {
    pub fn new(health: i32, field: Vec<GameItem>, name: String) -> PlayerState {
        let triggers = field
            .iter()
            .flat_map(|item| item.triggers.clone())
            .collect_vec();
        PlayerState {
            name,
            health,
            max_health: health,
            shield: 0,
            poison: 0,
            burn: 0,
            field,
            triggers,
        }
    }

    pub fn burn_damage(&mut self) {
        if self.burn > 0 {
            println!("Burn damage, health now {}", self.health);
            self.take_damage(self.burn, "Burn");
            self.burn -= 1;
        }
    }

    pub fn poison_damage(&mut self) {
        self.take_damage(self.poison, "Poison");
    }

    pub fn take_damage(&mut self, damage: i32, source: &str) {
        if damage <= 0 {
            return;
        }
        let health_before = self.health.clone();
        let shield_before = self.shield.clone();

        if self.shield > 0 {
            let shield_damage = damage - self.shield;
            if shield_damage > 0 {
                self.shield = 0;
                self.health -= shield_damage;
            } else {
                self.shield -= damage;
            }
            println!(
                "- DMG: {} Health/shield before/after ({}/{}) ({},{}) - {}",
                damage, health_before, shield_before, self.health, self.shield, source
            );
            return;
        }
        self.health -= damage;
        println!(
            "- DMG: {} Health/shield before/after ({}/{}) ({},{}) - {}",
            damage, health_before, shield_before, self.health, self.shield, source
        );
    }

    fn trigger_healing(&mut self, heal: i32, opponent: &mut PlayerState) {
        let triggers = self
            .triggers
            .iter()
            .filter(|trigger| trigger.trigger_type == TriggerType::OnHealing)
            .cloned()
            .collect_vec();

        for trigger in triggers {
            let item = self
                .field
                .iter_mut()
                .find(|item| item.id == trigger.item_id)
                .unwrap();
            (trigger.trigger)(ItemAction::Heal(heal), self, opponent)
        }
    }

    pub fn tick(&mut self, opponent: &mut PlayerState, amount: i32) {
        let mut actions = Vec::new();

        for item in self.field.iter_mut() {
            item.item_tick(amount);
        }

        let items_to_execute = self
            .field
            .iter_mut()
            .filter(|item| item.ticks_left <= 0)
            .collect_vec();

        for item in items_to_execute {
            for action in item.actions.iter() {
                actions.push(action.clone());
            }
            item.ticks_left = item.cooldown * 10;
        }

        for action in actions {
            match action {
                ItemAction::Damage(damage) => {
                    opponent.take_damage(damage, "Item");
                }
                ItemAction::Burn(damage) => {
                    println!("{}: Applying {} burn", self.name, damage);
                    opponent.burn += damage;
                }
                ItemAction::Poison(damage) => {
                    println!("{}: Applying {} poison", self.name, damage);
                    opponent.poison += damage;
                }
                ItemAction::Shield(shield) => {
                    println!("{}: Applying {} shield", self.name, shield);
                    self.shield += shield;
                }
                ItemAction::Heal(heal) => {
                    self.health += heal;
                    if self.health > self.max_health {
                        self.health = self.max_health;
                    }
                    println!(
                        "{}: Healing for {}, health now {}",
                        self.name, heal, self.health
                    );
                    self.trigger_healing(heal, opponent);
                }
                ItemAction::Slow(items_count, slow_amount) => {
                    let mut rng = rand::rng();
                    let mut indexes = (0..opponent.field.len()).collect_vec();
                    indexes.shuffle(&mut rng);
                    for i in 0..items_count {
                        if let Some(item) = opponent.field.get_mut(indexes[i]) {
                            if item.slow_ticks_left.is_some() {
                                item.slow_ticks_left =
                                    Some(item.slow_ticks_left.unwrap() + slow_amount);
                            } else {
                                item.slow_ticks_left = Some(slow_amount);
                            }
                        }
                    }
                }
                ItemAction::IncreaseAllDamage(amount) => {
                    self.field.iter_mut().for_each(|item| {
                        item.actions.iter_mut().for_each(|action| {
                            if let ItemAction::Damage(damage) = action {
                                *damage += amount;
                            }
                        });
                    });
                }
                ItemAction::IncreaseAllHealing(amount) => {
                    self.field.iter_mut().for_each(|item| {
                        item.actions.iter_mut().for_each(|action| {
                            if let ItemAction::Heal(heal) = action {
                                *heal += amount;
                            }
                        });
                    });
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TriggerType {
    OnGameStart,
    OnHealing,
}

#[derive(Clone, Debug)]
pub struct ItemTrigger {
    pub item_id: i32,
    pub trigger_type: TriggerType,
    pub trigger: fn(ItemAction, &mut PlayerState, &mut PlayerState),
}

#[derive(Clone, Debug)]
pub struct GameItem {
    pub name: String,
    pub id: i32,
    pub crit_chance: i32,
    pub cooldown: i32,
    pub cooldown_modifier: i32,
    pub ammo: Option<i32>,
    pub ticks_left: i32,
    pub actions: Vec<ItemAction>,
    pub triggers: Vec<ItemTrigger>,
    pub slow_ticks_left: Option<i32>,
    pub haste_ticks_left: Option<i32>,
}

impl GameItem {
    pub fn new(
        name: &str,
        crit_chance: i32,
        cooldown: i32,
        ammo: Option<i32>,
        actions: Vec<ItemAction>,
    ) -> GameItem {
        GameItem {
            name: name.to_string(),
            id: 1,
            crit_chance,
            cooldown: cooldown + 20,
            cooldown_modifier: 0,
            ammo,
            ticks_left: cooldown * 10,
            actions,
            triggers: Vec::new(),
            slow_ticks_left: None,
            haste_ticks_left: None,
        }
    }

    pub fn new_with_triggers(
        name: &str,
        crit_chance: i32,
        cooldown: i32,
        ammo: Option<i32>,
        actions: Vec<ItemAction>,
        triggers: Vec<ItemTrigger>,
    ) -> GameItem {
        GameItem {
            name: name.to_string(),
            id: 1,
            crit_chance,
            cooldown: cooldown,
            cooldown_modifier: cooldown,
            ammo,
            ticks_left: cooldown * 10,
            actions,
            triggers,
            slow_ticks_left: None,
            haste_ticks_left: None,
        }
    }

    pub fn item_tick(&mut self, amount: i32) -> &mut GameItem {
        let is_slowed = self.slow_ticks_left.is_some();
        let is_hasted = self.haste_ticks_left.is_some();

        let mut amount_to_tick = amount;
        if is_slowed {
            amount_to_tick /= 2;
        }
        if is_hasted {
            amount_to_tick *= 2;
        }

        self.ticks_left -= amount_to_tick;

        if let Some(slow_ticks_left) = self.slow_ticks_left {
            if slow_ticks_left - amount <= 0 {
                self.slow_ticks_left = None;
            } else {
                self.slow_ticks_left = Some(slow_ticks_left - amount);
            }
        }
        if let Some(haste_ticks_left) = self.haste_ticks_left {
            if haste_ticks_left - amount <= 0 {
                self.haste_ticks_left = None;
            } else {
                self.haste_ticks_left = Some(haste_ticks_left - amount);
            }
        }
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemAction {
    Damage(i32),
    Burn(i32),
    Poison(i32),
    Shield(i32),
    Heal(i32),
    Slow(usize, i32),
    IncreaseAllDamage(i32),
    IncreaseAllHealing(i32),
}
