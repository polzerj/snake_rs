use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::{
    config::GameConfig,
    game::Game,
    input::{InputAction, InputHandler},
    renderer::TuiRenderer,
    sound::SoundSystem,
};

pub struct App<I: InputHandler, S: SoundSystem> {
    game: Game,
    config: GameConfig,
    renderer: TuiRenderer,
    input_handler: I,
    sound_system: S,
    should_quit: bool,
}

impl<I: InputHandler, S: SoundSystem> App<I, S> {
    pub fn new(config: GameConfig, input_handler: I, sound_system: S) -> Self {
        let mut game = Game::new(config.board_width, config.board_height);
        game.set_wall_wrapping(config.wall_wrapping);
        Self {
            game,
            config,
            renderer: TuiRenderer::new(),
            input_handler,
            sound_system,
            should_quit: false,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();
        let mut direction_store_next_tick = None;

        loop {
            terminal.draw(|f| {
                self.renderer.draw_frame(f, &self.game, &self.config);
            })?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Ok(action) = self.input_handler.handle_input(event::read()?) {
                    match action {
                        InputAction::Move(direction) => {
                            // Only allow one direction change per tick
                            if direction_store_next_tick.is_none() {
                                self.game.set_direction(direction);
                            }
                            direction_store_next_tick = Some(direction);
                        }
                        InputAction::Pause => {
                            self.game.toggle_pause();
                        }
                        InputAction::Restart => {
                            self.game.reset();
                        }
                        InputAction::Quit => {
                            self.should_quit = true;
                        }
                        InputAction::None => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                let game_event = self.game.update();
                self.sound_system.play_sound(game_event);

                // Update high score if game over
                if matches!(game_event, crate::game::GameEvent::GameOver) {
                    self.config.update_high_score(self.game.score());
                }

                last_tick = Instant::now();
                self.game.set_direction(
                    direction_store_next_tick
                        .take()
                        .unwrap_or(self.game.snake().direction()),
                );
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
}

pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
