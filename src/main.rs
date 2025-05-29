use clap::Parser;
use ratatui::style::Color;
use snake_rs::{app, config, input, sound};
use std::io;

use app::{App, restore_terminal, setup_terminal};
use config::GameConfig;
use input::CrosstermInputHandler;
use sound::ConsoleSoundSystem;

/// A terminal-based Snake game written in Rust
#[derive(Parser, Debug)]
#[command(name = "snake_rs")]
#[command(about = "A terminal-based Snake game")]
#[command(version = "1.0.0")]
struct Args {
    /// Disable sound effects
    #[arg(long = "no-sound", short = 'm')]
    no_sound: bool,

    /// Disable wall wrapping (snake dies when hitting walls)
    #[arg(long = "solid-walls", short = 's')]
    solid_walls: bool,

    /// Disable colors
    #[arg(long = "no-color")]
    no_color: bool,

    /// Board width
    #[arg(long, default_value = "30")]
    width: u16,

    /// Board height
    #[arg(long, default_value = "20")]
    height: u16,
}

fn main() -> Result<(), io::Error> {
    // Configure the game - you can modify these settings
    let args = Args::parse();

    let wall_wrapping = !args.solid_walls; // Enable or disable wall wrapping based on CLI argument

    let config = GameConfig::new(args.width, args.height) // Board size: 30x20
        .with_sound(!args.no_sound) // Enable or disable console bell sounds based on CLI argument
        .with_colors(!args.no_color) // Enable or disable colors based on CLI argument
        .with_wall_wrapping(wall_wrapping)
        .with_snake_color(Color::Green) // Snake color
        .with_wall_color(if wall_wrapping {
            Color::LightGreen
        } else {
            Color::Red
        }) // Wall color
        .with_food_color(Color::LightRed) // Food color
        .with_border_color(Color::LightCyan)
        .with_background_color(Color::Black); // Background color

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
