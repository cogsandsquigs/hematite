// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Snake logic and helper functions.
//
// To get you started we've included code to prevent your Snake from moving backwards.
// For more info see docs.battlesnake.com

use crate::{configuration::Config, engine::Engine, game::GameState};
use log::{info, warn};
use serde_json::{json, Value};
use std::collections::HashMap;

/// The structure that handles all the `GET`/`POST` request logic for the game.
#[derive(Clone)]
pub struct Server {
    /// The configuration for the battlesnake itself and/or the engine. Loaded when
    /// a new server is created.
    config: Config,

    /// A map of game IDs to `Engine` instances, which handle the game logic.
    games: HashMap<String, Engine>,
}

/// Public API for `Server`.
impl Server {
    /// Creates a new `Server` instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            config: Config::load(),
            games: HashMap::new(),
        }
    }

    // info is called when you create your Snake on play.battlesnake.com
    // and controls your Snake's appearance
    // TIP: If you open your Snake URL in a browser you should see this data
    pub fn info(&self) -> Value {
        info!("INFO");

        // The author of the Snake - A.K.A. me!
        let author = self.config.battlesnake_username.as_str();

        // If the `debug_assertions` feature is enabled, the snake will be pink, showing that it is
        // in debug/development mode. Otherwise, it will be its usual color, showing that it is in
        // release mode.
        let color = if cfg!(debug_assertions) {
            self.config
                .snake
                .debug_color
                .as_deref()
                .unwrap_or("#ff00ff")
        } else {
            self.config.snake.color.as_str()
        };

        // The head of the Snake.
        let head = self.config.snake.head.as_str();

        // The tail of the Snake.
        let tail = self.config.snake.tail.as_str();

        // The version of the Snake - A.K.A. the version of the crate.
        let version = env!("CARGO_PKG_VERSION");

        json!({
            "apiversion": "1",
            "author": author,
            "color": color,
            "head": head,
            "tail": tail,
            "version": version,
        })
    }

    // start is called when your Snake begins a game
    pub fn start(&mut self, state: &GameState) {
        let id = &state.game.id;

        info!("Starting game '{id}'!");

        let engine = Engine::new(self.config.engine, state.clone());

        self.games.insert(id.clone(), engine);
    }

    // end is called when your Snake finishes a game
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
        let engine = if let Some(engine) = self.games.get_mut(id) {
            engine
        } else {
            warn!("No engine found for game '{id}'!");
            warn!("Creating new engine...");
            let engine = Engine::new(self.config.engine, state.clone());
            self.games.insert(id.clone(), engine);
            self.games.get_mut(id).unwrap()
        };

        engine.update(state.clone());
        let chosen = engine.get_move();

        info!("Moving {}", chosen);

        json!({ "move": chosen })
    }
}
