use ggez::{ContextBuilder, GameResult, event};
mod assets;
mod food;
mod game_state;
mod snake;
mod teleportation;
mod utils;
mod wall;

use game_state::GameState;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("snake_game", "gufanto")
        .add_resource_path(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources"))
        .build()?;

    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
