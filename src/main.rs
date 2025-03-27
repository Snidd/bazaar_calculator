fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use bazaarcalculator::{
        cards::*,
        example_players::{example_kripp1, example_player1},
        game_state::{GameItem, ItemAction, PlayerState},
        monsters::rogue_scrapper,
    };

    #[test]
    fn test_example_player1() {
        let player = example_player1();
        assert_eq!(player.health, 370);
        assert_eq!(player.shield, 0);
        assert_eq!(player.poison, 0);
        assert_eq!(player.burn, 0);
        assert_eq!(player.field.len(), 3);
    }

    #[test]
    fn test_rogue_scrapper() {
        let player = rogue_scrapper();
        assert_eq!(player.health, 450);
        assert_eq!(player.shield, 0);
        assert_eq!(player.poison, 0);
        assert_eq!(player.burn, 0);
        assert_eq!(player.field.len(), 4);
    }

    #[test]
    fn example_game_50() {
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(example_player1(), rogue_scrapper()),
        );

        assert_eq!(game_engine.game_state.player1.health, 370);
        assert_eq!(game_engine.game_state.player2.health, 450);

        for _ in 0..50 {
            game_engine.tick();
        }

        assert_eq!(game_engine.game_state.current_tick, 500);
        assert_eq!(game_engine.game_state.player1.health, 360);
        assert_eq!(game_engine.game_state.player2.health, 410);
        assert_eq!(game_engine.game_state.player2.shield, 30);
    }

    #[test]
    fn example_game_100() {
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(example_player1(), rogue_scrapper()),
        );

        assert_eq!(game_engine.game_state.player1.health, 370);
        assert_eq!(game_engine.game_state.player2.health, 450);

        for _ in 0..100 {
            game_engine.tick();
        }

        assert_eq!(game_engine.game_state.current_tick, 1000);
        assert_eq!(game_engine.game_state.player1.health, 260);
        assert_eq!(game_engine.game_state.player2.health, 410);
        assert_eq!(game_engine.game_state.player2.shield, 30);
    }

    #[test]
    fn example_game_complete() {
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(example_player1(), rogue_scrapper()),
        );

        game_engine.tick_until_winner();

        assert_eq!(game_engine.game_state.current_tick, 3500);
        assert_eq!(game_engine.game_state.player1.health, 0);
        assert_eq!(game_engine.game_state.player2.health, 410);
    }

    #[test]
    fn simple_damage_test() {
        let player1 = PlayerState::new(200, vec![junkyard_club()], "P1".to_string());
        let player2 = PlayerState::new(
            200,
            vec![GameItem::new(
                "Test",
                0,
                20,
                None,
                vec![ItemAction::Damage(10)],
            )],
            "P2".to_string(),
        );
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(player1, player2),
        );

        game_engine.tick();

        assert_eq!(game_engine.game_state.current_tick, 10);
        assert_eq!(game_engine.game_state.player1.health, 200);
        assert_eq!(game_engine.game_state.player2.health, 200);

        for _ in 0..109 {
            game_engine.tick();
        }

        assert_eq!(game_engine.game_state.current_tick, 1100);
        assert_eq!(game_engine.game_state.player1.health, 150);
        assert_eq!(game_engine.game_state.player2.health, 140);
    }

    #[test]
    fn simple_weight_test() {
        let player1 = PlayerState::new(200, vec![], "P1".to_string());
        let player2 = PlayerState::new(
            200,
            vec![
                GameItem::new("TestDamage", 0, 60, None, vec![ItemAction::Damage(10)]),
                GameItem::new("TestHeal", 0, 60, None, vec![ItemAction::Heal(10)]),
                Weights::new(GameItemLevel::Silver),
            ],
            "P2".to_string(),
        );
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(player1, player2),
        );

        for _ in 0..50 {
            game_engine.tick();
        }

        assert_eq!(game_engine.game_state.current_tick, 500);
        assert_eq!(game_engine.game_state.player1.health, 200);
        assert_eq!(game_engine.game_state.player2.health, 200);

        let test_dmg = game_engine
            .game_state
            .player2
            .field
            .iter()
            .find(|item| item.name == "TestDamage")
            .unwrap();

        assert_eq!(test_dmg.actions[0], ItemAction::Damage(25));

        let test_heal = game_engine
            .game_state
            .player2
            .field
            .iter()
            .find(|item| item.name == "TestHeal")
            .unwrap();

        assert_eq!(test_heal.actions[0], ItemAction::Heal(25));
    }

    #[test]
    fn triggered_weight_test() {
        let player1 = PlayerState::new(200, vec![], "P1".to_string());
        let mut player2 = PlayerState::new(
            200,
            vec![
                GameItem::new("TestDamage", 0, 60, None, vec![ItemAction::Damage(10)]),
                GameItem::new("TestHeal", 0, 10, None, vec![ItemAction::Heal(10)]),
                Weights::new(GameItemLevel::Silver),
            ],
            "P2".to_string(),
        );
        player2.health = 180;
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(player1, player2),
        );

        for _ in 0..40 {
            game_engine.tick();
        }

        assert_eq!(game_engine.game_state.current_tick, 400);
        assert_eq!(game_engine.game_state.player1.health, 200);
        assert_eq!(game_engine.game_state.player2.health, 200);

        let test_dmg = game_engine
            .game_state
            .player2
            .field
            .iter()
            .find(|item| item.name == "TestDamage")
            .unwrap();

        assert_eq!(test_dmg.actions[0], ItemAction::Damage(25));

        let test_heal = game_engine
            .game_state
            .player2
            .field
            .iter()
            .find(|item| item.name == "TestHeal")
            .unwrap();

        assert_eq!(test_heal.actions[0], ItemAction::Heal(25));
    }

    #[test]
    fn kripp1_test() {
        let player1 = example_kripp1();
        let player2 = rogue_scrapper();
        let mut game_engine = bazaarcalculator::game_engine::GameEngine::new(
            bazaarcalculator::game_state::GameState::new(player1, player2),
        );
        game_engine.tick_until_winner();

        assert_eq!(game_engine.game_state.player2.health, -22);
    }
}
