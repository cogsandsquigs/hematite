use super::Simulation;
use crate::objects::{
    moves::Move,
    point::Point,
    snake::{Snake, SnakeID},
};
use itertools::Itertools;
use rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};

impl Simulation {
    /// Checks if the game is terminated.
    pub fn is_over(&self) -> bool {
        self.alive_snakes.len() <= 1 || self.dead_snakes.contains_key(&self.snake_id)
    }

    /// Returns if we won the game.
    pub fn did_win(&self) -> bool {
        self.is_over() && self.alive_snakes.keys().contains(&self.snake_id)
    }

    /// Moves a snake in the simulation. Does nothing if the snake does not exist or is dead.
    pub fn move_snake(&mut self, id: SnakeID, move_: Move) {
        // The snake to move.
        let Some(snake) = self.alive_snakes.get_mut(&id) else {
            return;
        };

        // The snake's head, before the move.
        let head = snake.head;
        // And the new head, after it
        let new_head = move_.to_point(&head);
        // If the snake ate food or not.
        let ate_food = self.ate_food.contains(&id);

        // Insert the new head into the snake.
        snake.head = new_head;
        snake.body.insert(0, new_head);

        // If the snake ate food previously, and is less than 3 long, then we don't want to remove the tail.
        if !ate_food && snake.length < 3 {
            snake.body.pop();
        }
        // Otherwise, we increase the snake's length by one.
        else {
            snake.length += 1;
        }

        // Decrease the snake's health by one, or by more if it's in a hazard.
        snake.health -= if self.hazards.contains(&snake.head) {
            self.rules.settings.hazard_damage_per_turn
        } else {
            1
        } as i32;

        // If the snake ate food, then we add it to the set of snakes who ate food.
        if self.food.contains(&new_head) {
            self.ate_food.insert(id);
            self.food.remove(&new_head);
            snake.health += 1;
        }
        // Otherwise, if the snake didn't eat food, and it was in the set of snakes who ate food previously,
        // then we remove it from said set. Note that we only get here if the snake didn't eat food.
        else if !ate_food {
            self.ate_food.remove(&id);
        }
    }

    /// Removes all the terminated snakes from the simulation, and put them in the
    /// dead snakes set.
    pub fn remove_dead_snakes(&mut self) {
        let dead_snakes = self
            .alive_snakes
            .iter()
            .filter(|(_, snake)| self.will_die(snake))
            .map(|(id, _)| *id)
            .collect_vec();

        for id in dead_snakes {
            let snake = self
                .alive_snakes
                .remove(&id)
                .expect("The snake should exist!");
            self.dead_snakes.insert(id, snake);
        }
    }

    /// Spawn food in the simulation.
    pub fn spawn_food(&mut self) {
        let mut rng = SmallRng::from_entropy();

        if self.food.len() < self.rules.settings.minimum_food as usize
            || rng.gen_bool(self.rules.settings.food_spawn_chance)
        {
            let mut point = Point::new(
                rng.gen_range(0..self.width as i32),
                rng.gen_range(0..self.height as i32),
            );

            // While the point is in a snake, or other food, generate a new point.
            // TODO: Make this more efficient.
            while self
                .alive_snakes
                .values()
                .flat_map(|snake| snake.body.iter())
                .contains(&point)
                || self.food.contains(&point)
            {
                point = Point::new(
                    rng.gen_range(0..self.width as i32),
                    rng.gen_range(0..self.height as i32),
                );
            }

            self.food.insert(point);
        }
    }

    /// Check if a snake will die in the next turn. The input is the head of the snake.
    pub fn will_die(&self, snake: &Snake) -> bool {
        self.is_in_wall(&snake.head) || self.is_in_snake(snake) || snake.health <= 0
    }

    /// Check if a snake's head is a wall.
    pub fn is_in_wall(&self, head: &Point) -> bool {
        head.x < 0 || head.x >= self.width as i32 || head.y < 0 || head.y >= self.height as i32
    }

    /// Check if a point is a snake, from the perspective of a snake.
    /// TODO: Allow a move inside a snake's tail IF it is greater than 2 long
    /// OR if it is not consuming food.
    pub fn is_in_snake(&self, snake: &Snake) -> bool {
        for other in self
            .alive_snakes
            .values()
            .filter(|other| other.id != snake.id)
        {
            // If the snake is in the other snake's body (excluding the tail because it
            // moves out of the way), then it is in a snake.
            if other.body[..other.body.len() - 1].contains(&snake.head) {
                return true;
            }
        }

        // If we haven't returned true by now, then the point is not in a snake's body.
        false
    }

    /// Gets all the good moves a snake can make. A good move is one where it avoids both
    /// the walls, and itself. `is_node_move` is a boolean that determines if the snake can
    /// consider the game state (including other snakes) or not. This is usually true when
    /// getting good moves outside of a simulation (during MCTS), and false when getting good
    /// moves inside a simulation (where we can consider other snake's moves)
    pub fn good_moves(&self, snake: &Snake, is_node_move: bool) -> Vec<Move> {
        Move::all()
            .iter()
            .filter(|move_| {
                let new_head = move_.to_point(&snake.head);
                !self.is_in_wall(&new_head) && !snake.body.contains(&new_head) && {
                    // If we are not in a node move, then we check if the snake is in a snake.
                    // Otherwise, we just return true, because we can't consider other snakes.
                    // That could lead to a scenario where we could run into a snake's tail in
                    // one round of MCTS, but not in the next (due to food spawning), and then
                    // we would think that the move is bad, when it is not.
                    if !is_node_move {
                        !self.is_in_snake(snake)
                    } else {
                        true
                    }
                }
            })
            .copied()
            .collect()
    }

    /// Gets a random good move for a snake ID. This is used for the random bot. Panics if the
    /// snake doesn't exist.
    pub fn random_good_move(&self, id: &SnakeID) -> Move {
        let snake = self.alive_snakes.get(id).expect("The snake should exist!");
        let good_moves = self.good_moves(snake, false);
        let mut rng = SmallRng::from_entropy();

        *good_moves
            .choose(&mut rng)
            // If there are no good moves, then we just return Up. This is not random, but
            // in this case, it doesn't matter. The snake will die anyway.
            .unwrap_or(&Move::Up)
    }
}
