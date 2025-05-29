use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct GameConfig {
    pub board_width: u16,
    pub board_height: u16,
    pub enable_sound: bool,
    pub enable_colors: bool,
    pub wall_wrapping: bool,
    pub snake_color: Color,
    pub food_color: Color,
    pub wall_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub high_score: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            board_width: 40,
            board_height: 20,
            enable_sound: true,
            enable_colors: true,
            wall_wrapping: true,
            snake_color: Color::Green,
            food_color: Color::Red,
            wall_color: Color::Gray,
            background_color: Color::Black,
            border_color: Color::White,
            high_score: 0,
        }
    }
}

impl GameConfig {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            board_width: width,
            board_height: height,
            ..Default::default()
        }
    }

    pub fn with_sound(mut self, enable: bool) -> Self {
        self.enable_sound = enable;
        self
    }

    pub fn with_colors(mut self, enable: bool) -> Self {
        self.enable_colors = enable;
        self
    }

    pub fn with_wall_wrapping(mut self, enable: bool) -> Self {
        self.wall_wrapping = enable;
        self
    }

    pub fn with_snake_color(mut self, color: Color) -> Self {
        self.snake_color = color;
        self
    }

    pub fn with_food_color(mut self, color: Color) -> Self {
        self.food_color = color;
        self
    }

    pub fn with_wall_color(mut self, color: Color) -> Self {
        self.wall_color = color;
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn update_high_score(&mut self, score: u32) {
        if score > self.high_score {
            self.high_score = score;
        }
    }
}
