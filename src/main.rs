use ratatui::style::Color;
use snake_rs::{app, config, input, sound};
use std::io;

use app::{App, restore_terminal, setup_terminal};
use config::GameConfig;
use input::CrosstermInputHandler;
use sound::ConsoleSoundSystem;

fn main() -> Result<(), io::Error> {
    // Configure the game - you can modify these settings
    let config = GameConfig::new(30, 20) // Board size: 30x20
        .with_sound(true) // Enable console bell sounds
        .with_colors(true) // Enable colors
        .with_wall_wrapping(true)
        .with_snake_color(Color::Green) // Snake color
        .with_food_color(Color::Red); // Food color

    // Create dependencies
    let input_handler = CrosstermInputHandler::new();
    let sound_system = ConsoleSoundSystem::new(config.enable_sound);

    // Create and configure the application
    let mut app = App::new(config, input_handler, sound_system);

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Run the application
    let result = app.run(&mut terminal);

    // Restore terminal
    restore_terminal()?;

    // Print final message
    println!("Thanks for playing Snake!");

    result
}
