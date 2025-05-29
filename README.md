# Snake Game (TUI)

A terminal-based Snake game written in Rust using `ratatui` for the TUI interface, following SOLID principles and dependency injection patterns.

## Features

- **Configurable game board size** - Set custom width and height
- **Optional sound effects** - Console bell alerts for eating food and game over
- **Optional colors** - Customizable color scheme or monochrome mode
- **Wall wrapping** - Optional feature allowing snake to pass through walls and appear on the opposite side
- **Score tracking** - Points awarded for eating food
- **Length tracking** - Shows current snake length
- **Pause/Resume functionality** - Space bar to pause/resume
- **Game restart** - R key to restart the game

## Architecture

The game is structured with clear separation of concerns:

- **`app.rs`** - Main application orchestrator with dependency injection
- **`config.rs`** - Game configuration and settings
- **`game.rs`** - Core game logic and state management
- **`input.rs`** - Input handling abstraction
- **`renderer.rs`** - Rendering interface with TUI implementation
- **`sound.rs`** - Sound system abstraction with console implementation


## Controls

- **Arrow Keys** - Move the snake (Up, Down, Left, Right)
- **Space** - Pause/Resume the game
- **R** - Restart the game
- **Q/Esc** - Quit the game

## Configuration

The game can be configured in `main.rs`:

```rust
let config = GameConfig::new(30, 20)  // Board size: 30x20
    .with_sound(true)                 // Enable console bell sounds
    .with_colors(true)                // Enable colors
    .with_wall_wrapping(true)         // Enable wall wrapping (snake passes through walls)
    .with_snake_color(Color::Green)   // Snake color
    .with_food_color(Color::Red);     // Food color
```

### Wall Wrapping

When wall wrapping is enabled (`with_wall_wrapping(true)`), the snake can pass through the edges of the game board and appear on the opposite side. This creates a "wraparound" effect where:
- Moving off the right edge appears on the left side
- Moving off the left edge appears on the right side  
- Moving off the top edge appears at the bottom
- Moving off the bottom edge appears at the top

When disabled (`with_wall_wrapping(false)`), hitting a wall will end the game.

## Running the Game

```bash
cargo run
```

## Building

```bash
cargo build --release
```

## Dependencies

- `ratatui` - Terminal user interface library
- `crossterm` - Cross-platform terminal manipulation
- `rand` - Random number generation for food placement

## Game Rules

1. Control the snake using arrow keys
2. Eat food (◆) to grow and increase your score
3. Avoid hitting yourself
4. If wall wrapping is disabled, avoid hitting walls
5. If wall wrapping is enabled, you can pass through walls and appear on the opposite side
6. Score increases by 10 points for each food eaten
7. Game ends when the snake hits itself (or a wall if wrapping is disabled)

