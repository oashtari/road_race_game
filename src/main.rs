use rusty_engine::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    // game setup goes here

    let player1 = game.add_sprite("player1", SpritePreset::RacingCarGreen);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
}
