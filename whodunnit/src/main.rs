#![allow(warnings)]

use std::error::Error;

mod game;
use game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new()?;
    game.start()?;
    Ok(())
}
