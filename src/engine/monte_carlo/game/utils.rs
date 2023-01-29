use super::Simulation;
use crate::objects::{moves::Move, point::Point, snake::SnakeID};
use itertools::Itertools;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::collections::HashMap;

impl Simulation {
    /// Gets all the snake ids, including dead snakes.
    pub fn snake_ids(&self) -> Vec<SnakeID> {
        self.alive_snakes
            .keys()
            .chain(self.dead_snakes.keys())
            .copied()
            .collect()
    }

    /// Gets the previous move of all the snakes.
    pub fn previous_moves(&self) -> HashMap<SnakeID, Move> {
        self.alive_snakes
            .iter()
            .map(|(id, snake)| {
                let head = snake.borrow().body[0];
                let neck = snake.borrow().body[1];

                let move_ = Move::from_points(&head, &neck).expect(
                    "The snake's head and neck should be adjacent, so there should be a move!",
                );

                (*id, move_)
            })
            .collect()
    }

    /// Check if a snake ate food in the last turn.
    pub fn did_eat_food(&self, id: &SnakeID) -> bool {
        self.ate_food.contains(id)
    }

    /// Checks if the game is terminated.
    pub fn is_over(&self) -> bool {
        self.alive_snakes.len() <= 1 || self.dead_snakes.contains_key(&self.snake_id)
    }

    /// Returns if we won the game.
    pub fn did_win(&self) -> bool {
        self.is_over() && self.alive_snakes.keys().contains(&self.snake_id)
    }

    /// Moves a snake in the simulation.
    pub fn move_snake(&mut self, id: SnakeID, move_: Move) {
        let snake = self
            .alive_snakes
            .get(&id)
            .expect("The snake should exist in the simulation!");

        let head = snake.borrow().body[0];
        let new_head = move_.to_point(&head);

        snake.borrow_mut().body.insert(0, new_head);

        let ate_food = self.did_eat_food(&id);

        // If the snake ate food previously, then we don't want to remove the tail.
        if !ate_food {
            snake.borrow_mut().body.pop();
        }

        // If the snake ate food, then we add it to the set of snakes who ate food.
        if self.food.contains(&new_head) {
            self.ate_food.insert(id);
            self.food.remove(&new_head);
        }
        // Otherwise, if the snake didn't eat food, and it was in the set of snakes who ate food previously,
        // then we remove it from said set.
        else if !ate_food {
            self.ate_food.remove(&id);
        }
    }

    /// Removes all the terminated snakes from the simulation, and put them in the
    /// dead snakes set.
    pub fn remove_dead_snakes(&mut self) {
        for snake in self.alive_snakes.values() {
            let head = snake.borrow().head;

            // If the snake's head is a wall, or if the snake's head is in its body, or if it has no more
            // non-lethal moves, then it is dead.
            if self.will_die(&snake.borrow().id, &head)
                || Move::all()
                    .iter()
                    .all(|move_| self.will_die(&snake.borrow().id, &move_.to_point(&head)))
            {
                self.dead_snakes.insert(snake.borrow().id, *snake);
                self.alive_snakes.remove(&snake.borrow().id);
            }
        }
    }

    /// Spawn food in the simulation.
    pub fn spawn_food(&mut self) {
        if self.food.len() < self.rules.settings.minimum_food as usize
            || SmallRng::from_entropy().gen_bool(self.rules.settings.food_spawn_chance)
        {
            let mut rng = SmallRng::from_entropy();

            let mut point = Point::new(
                rng.gen_range(0..self.width as i32),
                rng.gen_range(0..self.height as i32),
            );

            self.food.insert(point);
        }
    }

    /// Spawn hazards in the simulation.
    /// TODO: Implement this.
    pub fn spawn_hazards(&mut self) {}

    /// Check if a point results in a termination if it is interpreted as a move.
    pub fn will_die(&self, id: &SnakeID, point: &Point) -> bool {
        self.is_wall(point) || self.is_snake(id, point)
    }

    /// Check if a point is a wall.
    pub fn is_wall(&self, point: &Point) -> bool {
        point.x < 0 || point.x >= self.width as i32 || point.y < 0 || point.y >= self.height as i32
    }

    /// Check if a point is a snake, from the perspective of a snake.
    pub fn is_snake(&self, id: &SnakeID, point: &Point) -> bool {
        let you = self
            .alive_snakes
            .get(id)
            .expect("The snake should exist in the simulation!")
            .borrow();

        for snake in self.alive_snakes.values() {
            let snake = snake.borrow();

            let length = snake.length;
            // The head of the snake.
            let head = snake.head;
            // All of the other snake's body except for the head and tail.
            let other_init = &snake.body[1..snake.body.len() - 1];
            // The head of the other snake.
            let other_head = snake.body[0];
            // The tail of the other snake.
            let other_tail = snake.tail();

            // If the point is in the snake's body, then it is a snake. Or, if the point is a head, and we
            // are less than the other snake, then it is a body part. Or, if the point is the tail, and the
            // snake is less than 3 tiles long or is moving into a food, then it is also counted as a body
            // part.
            if other_init.contains(point)
                || other_head == *point && (you.length <= length)
                || other_tail == *point
                    && (length < 3
                        || head
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
}
