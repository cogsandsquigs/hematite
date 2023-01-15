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

use crate::{
    board::{Battlesnake, Board},
    engine::Engine,
    game::{Game, GameState},
};
use log::info;
use rand::seq::SliceRandom;
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

        info!("Starting game!");
        info!("Game ID: {}", id);

        let engine = Engine::new(id.clone(), state.clone());

        self.games.insert(id.clone(), engine);
    }

    // end is called when your Battlesnake finishes a game
    pub fn end(&mut self, state: &GameState) {
        let id = &state.game.id;

        info!("Game over!");
        info!("Game ID: {}", id);

        self.games.remove(id);
    }

    // move is called on every turn and returns your next move
    // Valid moves are "up", "down", "left", or "right"
    // See https://docs.battlesnake.com/api/example-move for available data
    pub fn get_move(
        &mut self,
        _game: &Game,
        turn: &u32,
        _board: &Board,
        you: &Battlesnake,
    ) -> Value {
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
        .into_iter()
        .collect();

        // We've included code to prevent your Battlesnake from moving backwards
        let my_head = &you.body[0]; // Coordinates of your head
        let my_neck = &you.body[1]; // Coordinates of your "neck"

        if my_neck.x < my_head.x {
            // Neck is left of head, don't move left
            is_move_safe.insert("left", false);
        } else if my_neck.x > my_head.x {
            // Neck is right of head, don't move right
            is_move_safe.insert("right", false);
        } else if my_neck.y < my_head.y {
            // Neck is below head, don't move down
            is_move_safe.insert("down", false);
        } else if my_neck.y > my_head.y {
            // Neck is above head, don't move up
            is_move_safe.insert("up", false);
        }

        // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
        // let board_width = &board.width;
        // let board_height = &board.height;

        // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
        // let my_body = &you.body;

        // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
        // let opponents = &board.snakes;

        // Are there any safe moves left?
        let safe_moves = is_move_safe
            .into_iter()
            .filter(|&(_, v)| v)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        // Choose a random move from the safe ones
        let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

        // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
        // let food = &board.food;

        info!("Moving {}: {}", turn, chosen);

        json!({ "move": chosen })
    }
}
