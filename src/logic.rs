// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use crate::{engine::Engine, game::GameState};
use log::info;
use serde_json::{json, Value};
use std::collections::HashMap;

/// The structure that handles all the `GET`/`POST` request logic for the game.
#[derive(Clone)]
pub struct Logic {
    games: HashMap<String, Engine>,
}

/// Public API for `Logic`.
impl Logic {
    /// Creates a new `Logic` instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    // info is called when you create your Battlesnake on play.battlesnake.com
    // and controls your Battlesnake's appearance
    // TIP: If you open your Battlesnake URL in a browser you should see this data
    pub fn info(&self) -> Value {
        info!("INFO");

        json!({
            "apiversion": "1",
            "author": "cogsandsquigs",
            "color": "#000000",
            "head": "sand-worm",
            "tail": "sharp", // TODO: Choose tail
        })
    }

    // start is called when your Battlesnake begins a game
    pub fn start(&mut self, state: &GameState) {
        let id = &state.game.id;

        info!("Starting game '{id}'!");

        let engine = Engine::new(id.clone(), state.clone());

        self.games.insert(id.clone(), engine);
    }

    // end is called when your Battlesnake finishes a game
    pub fn end(&mut self, state: &GameState) {
        let id = &state.game.id;

        info!("Game '{id}' over!");

        self.games.remove(id);
    }

    // move is called on every turn and returns your next move
    // Valid moves are "up", "down", "left", or "right"
    // See https://docs.battlesnake.com/api/example-move for available data
    pub fn get_move(&mut self, state: &GameState) -> Value {
        info!("Calculating move...");

        let id = &state.game.id;
        let engine = self.games.get_mut(id).unwrap();

        engine.update(state.clone());
        let chosen = engine.get_move();

        info!("Moving {}", chosen);

        json!({ "move": chosen })
    }
}
