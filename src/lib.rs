// lib.rs - Library interface for snake_rs
pub mod app;
pub mod config;
pub mod game;
pub mod input;
pub mod renderer;
pub mod sound;

// Re-export commonly used items
pub use app::App;
pub use config::GameConfig;
pub use game::{Game, Direction, GameState, GameEvent, Position};
pub use input::{InputAction, InputHandler, CrosstermInputHandler};
pub use renderer::{Renderer, TuiRenderer};
pub use sound::{SoundSystem, ConsoleSoundSystem, NoSoundSystem};
