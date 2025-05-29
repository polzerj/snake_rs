use crate::game::Direction;
use crossterm::event::{Event, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputAction {
    Move(Direction),
    Pause,
    Restart,
    Quit,
    None,
}

pub trait InputHandler {
    type Error;

    fn handle_input(&self, event: Event) -> Result<InputAction, Self::Error>;
}

pub struct CrosstermInputHandler;

impl Default for CrosstermInputHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CrosstermInputHandler {
    pub fn new() -> Self {
        Self
    }
}

impl InputHandler for CrosstermInputHandler {
    type Error = std::io::Error;

    fn handle_input(&self, event: Event) -> Result<InputAction, Self::Error> {
        if let Event::Key(KeyEvent { code, .. }) = event {
            let action = match code {
                KeyCode::Up | KeyCode::Char('w') => InputAction::Move(Direction::Up),
                KeyCode::Down | KeyCode::Char('s') => InputAction::Move(Direction::Down),
                KeyCode::Left | KeyCode::Char('a') => InputAction::Move(Direction::Left),
                KeyCode::Right | KeyCode::Char('d') => InputAction::Move(Direction::Right),
                KeyCode::Char(' ') => InputAction::Pause,
                KeyCode::Char('r') | KeyCode::Char('R') => InputAction::Restart,
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => InputAction::Quit,
                _ => InputAction::None,
            };
            Ok(action)
        } else {
            Ok(InputAction::None)
        }
    }
}
