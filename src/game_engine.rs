use crate::game_state::GameState;

pub struct GameEngine {
    pub game_state: GameState,
}

pub const ONE_SECOND_IN_TICKS: i32 = 100;

impl GameEngine {
    pub fn new(game_state: GameState) -> GameEngine {
        GameEngine { game_state }
    }

    pub fn tick(&mut self) {
        // 100 is one second
        let amount_to_tick = ONE_SECOND_IN_TICKS / 10;
        self.game_state.current_tick += amount_to_tick;
        self.game_state
            .player1
            .tick(&mut self.game_state.player2, amount_to_tick);

        self.game_state
            .player2
            .tick(&mut self.game_state.player1, amount_to_tick);

        if self.game_state.current_tick % 100 == 0 {
            //burn damage
            self.game_state.player1.burn_damage();
            self.game_state.player2.burn_damage();
        }

        if self.game_state.current_tick % 200 == 0 {
            //poison damage
            self.game_state.player1.poison_damage();
            self.game_state.player2.poison_damage();
        }

        if self.game_state.current_tick > 3500 {
            if self.game_state.sandstorm_damage == 0 {
                self.game_state.sandstorm_damage = 1;
            } else {
                self.game_state.sandstorm_damage += 2;
            }
            self.game_state
                .player1
                .take_damage(self.game_state.sandstorm_damage, "Sandstorm");
            self.game_state
                .player2
                .take_damage(self.game_state.sandstorm_damage, "Sandstorm");
        }

        if self.game_state.player1.health < 0 {
            self.game_state.game_ended = true;
            self.game_state.winner = Some(self.game_state.player2.name.clone());
        }
        if self.game_state.player2.health < 0 {
            self.game_state.game_ended = true;
            let winner = match self.game_state.winner {
                Some(ref winner) => winner.clone(),
                None => "".to_string(),
            };
            self.game_state.winner = Some(format!("{} {}", winner, self.game_state.player1.name));
        }
    }

    pub fn tick_until_winner(&mut self) {
        while self.game_state.game_ended == false {
            self.tick();
        }
    }
}
