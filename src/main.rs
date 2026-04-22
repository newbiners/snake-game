use ggez::{event, ContextBuilder, GameResult};
mod game_state;
mod food;
mod snake;
mod utils;
mod wall;
mod teleportation;


use game_state::GameState;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("snake_game", "gufanto").build()?;

    let state = GameState::new();

    event::run(ctx, event_loop, state)
}
