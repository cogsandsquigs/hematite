use super::Engine;
use crate::game::{
    board::{Battlesnake, Board},
    moves::Move,
    point::Point,
};
use std::collections::HashSet;

/// API for gettting all the long-term safe moves for the engine's snake.
impl Engine {
    /// Get all the long-term safe moves for the engine
    pub fn engine_longevity_moves(&self, moves: HashSet<Move>) -> HashSet<Move> {
        self.snake_longevity_moves(moves, &self.you)
    }
}

/// API for getting all the long-term safe moves for any snake.
impl Engine {
    /// Gets all the long-term safe moves for `snake`.
    pub fn snake_longevity_moves(
        &self,
        moves: HashSet<Move>,
        snake: &Battlesnake,
    ) -> HashSet<Move> {
        self.snake_non_trapping_moves(moves, snake)
    }

    /// Gets all the moves that will not trap the snake in a space smaller than it. If there
    /// are no moves that will not trap the snake, returns the set of moves that have the
    /// most space to grow into -> prioritize longevity.
    fn snake_non_trapping_moves(
        &self,
        mut moves: HashSet<Move>,
        snake: &Battlesnake,
    ) -> HashSet<Move> {
        let head = &snake.head; // Coordinates of the head

        // Coordinates of all possible moves
        let points = moves
            .iter()
            .map(|&move_| (move_, move_.to_coord(head)))
            .collect::<HashSet<_>>();

        let mut largest_size = 0;
        let mut largest_set = HashSet::new();

        // Floodfill the board from each coordinate, and check if the number of filled
        // spaces is greater than (or equal to) the snake's length. If it isn't, remove
        // the move from the list of possible moves.
        // TODO: This is a bit inefficient, because it will floodfill the board multiple
        // times for the same coordinate. It would be better to floodfill the board once,
        // and then check each coordinate against the floodfill.
        for (move_, point) in points {
            let (filled, foods, _) = floodfill(&self.board, point);

            // If the number of filled spaces is greater than the largest number of filled
            // spaces, clear the set of moves and add the current move to it.
            match filled {
                filled if filled > largest_size => {
                    largest_size = filled;
                    largest_set.clear();
                    largest_set.insert(move_);
                }
                filled if filled == largest_size => {
                    largest_set.insert(move_);
                }
                _ => {}
            }

            // If the number of filled spaces is less than the snake's length, remove the
            // move from the set of possible moves. In this case, we subtract the number
            // of foods from the number of filled spaces because it grows the snake by 1.
            if filled - foods < snake.length {
                moves.remove(&move_);
            }
        }

        // If there are no moves that will not trap the snake, return the set of moves
        // that have the most space to grow into -> prioritize longevity.
        if moves.is_empty() {
            largest_set
        } else {
            moves
        }
    }

    /// Avoids any spaces that other snake's heads can move into, if the other snake is
    /// longer than the snake.
    pub fn snake_head_avoiding_moves(
        &self,
        mut moves: HashSet<Move>,
        snake: &Battlesnake,
    ) -> HashSet<Move> {
        let head = &snake.head; // Coordinates of the head
        let length = snake.length; // Length of the snake

        // All the other snakes on the board
        let other_snakes = self
            .board
            .snakes
            .iter()
            .filter(|other| other.id != snake.id)
            .collect::<Vec<_>>();

        for move_ in moves.clone() {
            let point = move_.to_coord(head);

            // If the move is into a space that another snake's head can move into, and
            // the other snake is longer than the snake, remove the move from the set of
            // possible moves.
            if other_snakes.iter().any(|other| {
                other.length > length && self.board.ortho_neighbors(&other.head).contains(&point)
            }) {
                moves.remove(&move_);
            }
        }

        moves
    }
}

/// Floodfills the board from the given coordinates, and returns the number of spaces
/// that were filled, the number of foods that were found, as well as all the points
/// visited. This is the number of spaces that the snake can move into, accounting for
/// growth.
fn floodfill(board: &Board, point: Point) -> (u32, u32, HashSet<Point>) {
    let mut filled = 0;
    let mut foods = 0;
    let mut queue = Vec::new();
    let mut visited = HashSet::new();

    // Add the starting coordinate to the queue
    queue.push(point);

    // While there are still coordinates to check
    while let Some(point) = queue.pop() {
        // If the coordinate is already filled, or if it is part of a snake, skip it.
        if visited.contains(&point) || board.snakes.iter().any(|snake| snake.body.contains(&point))
        {
            continue;
        }

        // Add the coordinate to the list of filled coordinates
        visited.insert(point);

        // Add the coordinates of the adjacent spaces to the queue
        board
            .ortho_neighbors(&point)
            .into_iter()
            .for_each(|point| queue.push(point));

        // If the coordinate has food, skip it, as we don't want to count it
        if board.food.contains(&point) {
            foods += 1;
        }

        // Increment the number of filled spaces
        filled += 1;
    }

    (filled, foods, visited)
}
