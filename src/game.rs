use rand::{Rng, rngs::ThreadRng};
use std::collections::VecDeque;

const INITIAL_SNAKE_LENGTH: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
}

impl Snake {
    pub fn new(start_pos: Position) -> Self {
        let mut body = VecDeque::new();
        body.push_back(start_pos);
        Self {
            body,
            direction: Direction::Right,
        }
    }

    pub fn head(&self) -> Position {
        *self.body.front().unwrap()
    }

    pub fn body(&self) -> &VecDeque<Position> {
        &self.body
    }

    #[allow(dead_code)]
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        // Prevent snake from turning into itself
        if direction != self.direction.opposite() {
            self.direction = direction;
        }
    }

    pub fn move_forward(&mut self) -> Position {
        let head = self.head();
        let new_head = match self.direction {
            Direction::Up => Position::new(head.x, head.y.saturating_sub(1)),
            Direction::Down => Position::new(head.x, head.y + 1),
            Direction::Left => Position::new(head.x.saturating_sub(1), head.y),
            Direction::Right => Position::new(head.x + 1, head.y),
        };

        self.body.push_front(new_head);
        self.body.pop_back().unwrap()
    }

    pub fn move_forward_with_wrapping(&mut self, board_width: u16, board_height: u16) -> Position {
        let head = self.head();
        let new_head = match self.direction {
            Direction::Up => {
                if head.y == 0 {
                    Position::new(head.x, board_height - 1) // Wrap to bottom
                } else {
                    Position::new(head.x, head.y - 1)
                }
            }
            Direction::Down => {
                if head.y >= board_height - 1 {
                    Position::new(head.x, 0) // Wrap to top
                } else {
                    Position::new(head.x, head.y + 1)
                }
            }
            Direction::Left => {
                if head.x == 0 {
                    Position::new(board_width - 1, head.y) // Wrap to right
                } else {
                    Position::new(head.x - 1, head.y)
                }
            }
            Direction::Right => {
                if head.x >= board_width - 1 {
                    Position::new(0, head.y) // Wrap to left
                } else {
                    Position::new(head.x + 1, head.y)
                }
            }
        };

        self.body.push_front(new_head);
        self.body.pop_back().unwrap()
    }

    pub fn grow(&mut self, old_tail: Position) {
        self.body.push_back(old_tail);
    }

    pub fn check_self_collision(&self) -> bool {
        let head = self.head();
        self.body.iter().skip(1).any(|&pos| pos == head)
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

pub struct Game {
    snake: Snake,
    food: Position,
    score: u32,
    state: GameState,
    board_width: u16,
    board_height: u16,
    wall_wrapping: bool,
    rng: ThreadRng,
}

impl Game {
    pub fn new(board_width: u16, board_height: u16) -> Self {
        let start_pos = Position::new(board_width / 2, board_height / 3);
        let mut game = Self {
            snake: Snake::new(start_pos),
            food: Position::new(0, 0),
            score: 0,
            state: GameState::Playing,
            board_width,
            board_height,
            wall_wrapping: false, // Default to false for backward compatibility
            rng: rand::rng(),
        };
        game.grow_to_initial_length(INITIAL_SNAKE_LENGTH);
        game.spawn_food();
        game
    }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn food(&self) -> Position {
        self.food
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn set_wall_wrapping(&mut self, enabled: bool) {
        self.wall_wrapping = enabled;
    }

    #[allow(dead_code)]
    pub fn wall_wrapping(&self) -> bool {
        self.wall_wrapping
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.state == GameState::Playing {
            self.snake.set_direction(direction);
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            GameState::GameOver => {}
        }
    }
    fn grow_to_initial_length(&mut self, length: usize) {
        for _ in 0..length {
            let tail = self.snake.move_forward();
            self.snake.grow(tail);
        }
    }

    pub fn reset(&mut self) {
        let start_pos = Position::new(self.board_width / 2, self.board_height / 2);
        self.snake = Snake::new(start_pos);
        self.grow_to_initial_length(INITIAL_SNAKE_LENGTH);
        self.score = 0;
        self.state = GameState::Playing;
        // Note: wall_wrapping setting is preserved during reset
        self.spawn_food();
    }

    pub fn update(&mut self) -> GameEvent {
        if self.state != GameState::Playing {
            return GameEvent::None;
        }

        let old_tail = if self.wall_wrapping {
            self.snake
                .move_forward_with_wrapping(self.board_width, self.board_height)
        } else {
            self.snake.move_forward()
        };

        let head = self.snake.head();

        // Check wall collision only if wrapping is disabled
        if !self.wall_wrapping && self.is_out_of_bounds(head) {
            self.state = GameState::GameOver;
            return GameEvent::GameOver;
        }

        // Check self collision
        if self.snake.check_self_collision() {
            self.state = GameState::GameOver;
            return GameEvent::GameOver;
        }

        // Check food collision
        if head == self.food {
            self.snake.grow(old_tail);
            self.score += 10;
            self.spawn_food();
            return GameEvent::FoodEaten;
        }

        GameEvent::Moved
    }

    fn is_out_of_bounds(&self, pos: Position) -> bool {
        pos.x >= self.board_width || pos.y >= self.board_height
    }

    fn spawn_food(&mut self) {
        loop {
            let x = self.rng.random_range(0..self.board_width);
            let y = self.rng.random_range(0..self.board_height);
            let food_pos = Position::new(x, y);

            // Make sure food doesn't spawn on snake
            if !self.snake.body().contains(&food_pos) {
                self.food = food_pos;
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    None,
    Moved,
    FoodEaten,
    GameOver,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_wrapping_horizontal() {
        let mut game = Game::new(5, 5);
        game.set_wall_wrapping(true);

        // Position snake at the right edge
        game.snake.body.clear();
        game.snake.body.push_back(Position::new(4, 2));
        game.snake.set_direction(Direction::Right);

        // Move forward - should wrap to left side
        game.update();

        assert_eq!(game.snake.head().x, 0);
        assert_eq!(game.snake.head().y, 2);
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn test_wall_wrapping_vertical() {
        let mut game = Game::new(5, 5);
        game.set_wall_wrapping(true);

        // Position snake at the top edge
        game.snake.body.clear();
        game.snake.body.push_back(Position::new(2, 0));
        game.snake.set_direction(Direction::Up);

        // Move forward - should wrap to bottom side
        game.update();

        assert_eq!(game.snake.head().x, 2);
        assert_eq!(game.snake.head().y, 4);
        assert_eq!(game.state, GameState::Playing);
    }

    #[test]
    fn test_no_wall_wrapping() {
        let mut game = Game::new(5, 5);

        // Position snake at the right edge
        game.snake.body.clear();
        game.snake.body.push_back(Position::new(4, 2));
        game.snake.set_direction(Direction::Right);

        // Move forward - should cause game over
        game.update();

        assert_eq!(game.state, GameState::GameOver);
    }
}
