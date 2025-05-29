use crate::config::GameConfig;
use crate::game::{Game, GameState};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub trait Renderer {
    type Error;

    #[allow(dead_code)]
    fn render(&mut self, game: &Game, config: &GameConfig) -> Result<(), Self::Error>;
    #[allow(dead_code)]
    fn clear(&mut self) -> Result<(), Self::Error>;
}

pub struct TuiRenderer;

impl Default for TuiRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiRenderer {
    pub fn new() -> Self {
        Self
    }

    fn render_game_area(&self, f: &mut Frame, game: &Game, config: &GameConfig, area: Rect) {
        let outer_block = Block::default().title("Snake Game").borders(Borders::ALL);

        let outer_inner = outer_block.inner(area);
        f.render_widget(outer_block, area);

        // Calculate minimum space needed for the game board (plus border)
        let min_width = config.board_width * 2 + 2; // +2 for border
        let min_height = config.board_height + 2; // +2 for border

        // Check if terminal is too small
        if outer_inner.width < min_width || outer_inner.height < min_height {
            let message = format!(
                "Terminal too small!\nMinimum size: {}x{}\nCurrent size: {}x{}",
                min_width, min_height, outer_inner.width, outer_inner.height
            );
            let warning_paragraph = Paragraph::new(message)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(warning_paragraph, outer_inner);
            return;
        }

        // Calculate optimal cell size that fits within available space
        let max_cell_width = outer_inner.width / 2 / config.board_width;
        let max_cell_height = outer_inner.height / config.board_height;

        // Use the smaller dimension to maintain aspect ratio and ensure everything fits
        let cell_size = std::cmp::min(max_cell_width, max_cell_height);
        let cell_size = std::cmp::max(1, cell_size); // Minimum of 1

        // Calculate the exact game board dimensions
        let game_width = config.board_width * cell_size * 2;
        let game_height = config.board_height * cell_size;

        // Center the game board within the available outer inner area
        let offset_x = (outer_inner.width.saturating_sub(game_width + 2)) / 2; // +2 for game border
        let offset_y = (outer_inner.height.saturating_sub(game_height + 2)) / 2; // +2 for game border

        // Create the game board area with border
        let game_board_area = Rect::new(
            outer_inner.x + offset_x,
            outer_inner.y + offset_y,
            game_width + 2,
            game_height + 2,
        );

        // Draw the game board border
        let border_style = if config.enable_colors {
            Style::default().fg(config.border_color)
        } else {
            Style::default()
        };

        let game_block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = game_block.inner(game_board_area);
        f.render_widget(game_block, game_board_area);

        // Render snake
        let snake_style = if config.enable_colors {
            Style::default().fg(config.snake_color)
        } else {
            Style::default()
        };

        for (i, segment) in game.snake().body().iter().enumerate() {
            // Skip if position is out of bounds for the game board
            if segment.x >= config.board_width || segment.y >= config.board_height {
                continue;
            }

            let x = inner.x + (segment.x * cell_size * 2);
            let y = inner.y + (segment.y * cell_size);

            let symbol = if i == 0 { "●" } else { "○" }; // Head vs body

            // Create a cell area
            let segment_area = Rect::new(x, y, cell_size, cell_size);

            if segment_area.width > 0 && segment_area.height > 0 {
                let segment_widget = Paragraph::new(symbol)
                    .style(snake_style)
                    .alignment(Alignment::Center);
                f.render_widget(segment_widget, segment_area);
            }
        }

        // Render food
        let food_style = if config.enable_colors {
            Style::default().fg(config.food_color)
        } else {
            Style::default()
        };

        let food = game.food();

        // Skip if food position is out of bounds for the game board
        if food.x >= config.board_width || food.y >= config.board_height {
            return;
        }

        let food_x = inner.x + (food.x * cell_size * 2);
        let food_y = inner.y + (food.y * cell_size);

        let food_area = Rect::new(food_x, food_y, cell_size, cell_size);

        if food_area.width > 0 && food_area.height > 0 {
            let food_widget = Paragraph::new("◆")
                .style(food_style)
                .alignment(Alignment::Center);
            f.render_widget(food_widget, food_area);
        }
    }

    fn render_score_area(&self, f: &mut Frame, game: &Game, config: &GameConfig, area: Rect) {
        let border_color = if config.enable_colors {
            config.border_color
        } else {
            Color::White
        };

        let score_text = vec![
            Line::from(vec![
                Span::styled("Score: ", Style::default().fg(border_color)),
                Span::styled(
                    game.score().to_string(),
                    Style::default()
                        .fg(if config.enable_colors {
                            Color::Yellow
                        } else {
                            Color::White
                        })
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("High Score: ", Style::default().fg(border_color)),
                Span::styled(
                    config.high_score.to_string(),
                    Style::default()
                        .fg(if config.enable_colors {
                            Color::Magenta
                        } else {
                            Color::White
                        })
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Length: ", Style::default().fg(border_color)),
                Span::styled(
                    game.snake().len().to_string(),
                    Style::default()
                        .fg(if config.enable_colors {
                            Color::Cyan
                        } else {
                            Color::White
                        })
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let score_block = Block::default()
            .title("Stats")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let score_paragraph = Paragraph::new(score_text)
            .block(score_block)
            .alignment(Alignment::Left);

        f.render_widget(score_paragraph, area);
    }

    fn render_controls_area(&self, f: &mut Frame, config: &GameConfig, area: Rect) {
        let border_color = if config.enable_colors {
            config.border_color
        } else {
            Color::White
        };

        let controls_text = vec![
            Line::from("Arrow Keys: Move"),
            Line::from("Space: Pause/Resume"),
            Line::from("R: Restart"),
            Line::from("Q: Quit"),
        ];

        let controls_block = Block::default()
            .title("Controls")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let controls_paragraph = Paragraph::new(controls_text)
            .block(controls_block)
            .alignment(Alignment::Left);

        f.render_widget(controls_paragraph, area);
    }

    fn render_overlay(&self, f: &mut Frame, game: &Game, config: &GameConfig) {
        let area = f.area();

        match game.state() {
            GameState::Paused => {
                let popup_area = self.centered_rect(30, 20, area);
                f.render_widget(Clear, popup_area);

                let border_color = if config.enable_colors {
                    config.border_color
                } else {
                    Color::White
                };

                let pause_block = Block::default()
                    .title("PAUSED")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color));

                let pause_text = Paragraph::new("Press Space to resume")
                    .block(pause_block)
                    .alignment(Alignment::Center);

                f.render_widget(pause_text, popup_area);
            }
            GameState::GameOver => {
                let popup_area = self.centered_rect(40, 30, area);
                f.render_widget(Clear, popup_area);

                let border_color = if config.enable_colors {
                    Color::Red
                } else {
                    Color::White
                };

                let game_over_block = Block::default()
                    .title("GAME OVER")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color));

                let game_over_text = vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Final Score: ", Style::default()),
                        Span::styled(
                            game.score().to_string(),
                            Style::default()
                                .fg(if config.enable_colors {
                                    Color::Yellow
                                } else {
                                    Color::White
                                })
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(""),
                    Line::from("Press R to restart"),
                    Line::from("Press Q to quit"),
                ];

                let game_over_paragraph = Paragraph::new(game_over_text)
                    .block(game_over_block)
                    .alignment(Alignment::Center);

                f.render_widget(game_over_paragraph, popup_area);
            }
            GameState::Playing => {}
        }
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

impl Renderer for TuiRenderer {
    type Error = std::io::Error;

    fn render(&mut self, _game: &Game, _config: &GameConfig) -> Result<(), Self::Error> {
        // This will be called from the main application loop
        // The actual frame rendering is handled by the main loop
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Helper function for the main application to use
impl TuiRenderer {
    pub fn draw_frame(&self, f: &mut Frame, game: &Game, config: &GameConfig) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(50),    // Game area
                Constraint::Length(25), // Side panel
            ])
            .split(f.area());

        let side_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Score (increased for high score)
                Constraint::Min(8),    // Controls
            ])
            .split(chunks[1]);

        self.render_game_area(f, game, config, chunks[0]);
        self.render_score_area(f, game, config, side_chunks[0]);
        self.render_controls_area(f, config, side_chunks[1]);
        self.render_overlay(f, game, config);
    }
}
