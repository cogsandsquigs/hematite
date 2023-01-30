use super::Simulation;
use crate::objects::{
    moves::Move,
    point::Point,
    snake::{Snake, SnakeID},
};
use itertools::Itertools;
use rand::{rngs::SmallRng, Rng, SeedableRng};

impl Simulation {
    /// Gets an iterator of all the snakes, alive or dead.
    pub fn snakes(&self) -> impl Iterator<Item = &Snake> {
        self.alive_snakes.values().chain(self.dead_snakes.values())
    }

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
        if self.food.len() < self.rules.settings.minimum_food as usize
            || SmallRng::from_entropy().gen_bool(self.rules.settings.food_spawn_chance)
        {
            let mut rng = SmallRng::from_entropy();

            let point = Point::new(
                rng.gen_range(0..self.width as i32),
                rng.gen_range(0..self.height as i32),
            );

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
    pub fn is_in_snake(&self, snake: &Snake) -> bool {
        for other in self.alive_snakes.values() {
            // All of the other snake's body except for the head and tail.
            let other_body = &other.body[1..other.body.len() - 1];
            // The tail of the other snake.
            let other_tail = other.tail();

            // If the point is in the snake's body, then it is a snake. Or, if the point is a head, and we
            // are less than the other snake, then it is a body part. Or, if the point is the tail, and the
            // snake is less than 3 tiles long or is moving into a food, then it is also counted as a body
            // part.
            if other_body.contains(&snake.head)
                //|| (other.head == snake.head && (snake.length <= other.length))
                || other_tail == snake.head
                    && (snake.length < 3
                        || snake
                            .head
                            .neighbors()
                            .iter()
                            .all(|neighbor| !self.food.contains(neighbor)))
            {
                return true;
            }
        }

        // If we haven't returned true by now, then the point is not in a snake's body.
        false
    }

    /// Gets all the good moves a snake can make. A good move is one where it avoids both
    /// the walls, and itself.
    pub fn good_moves(&self, snake: &Snake) -> Vec<Move> {
        Move::all()
            .iter()
            .filter(|move_| {
                let new_head = move_.to_point(&snake.head);
                !self.is_in_wall(&new_head) && !snake.body.contains(&new_head)
            })
            .copied()
            .collect()
    }
}
