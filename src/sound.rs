use crate::game::GameEvent;

pub trait SoundSystem {
    fn play_sound(&self, event: GameEvent);
}

pub struct ConsoleSoundSystem {
    enabled: bool,
}

impl ConsoleSoundSystem {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl SoundSystem for ConsoleSoundSystem {
    fn play_sound(&self, event: GameEvent) {
        if !self.enabled {
            return;
        }

        match event {
            GameEvent::FoodEaten => {
                // Bell sound for eating food
                print!("\x07");
            }
            GameEvent::GameOver => {
                // Multiple beeps for game over
                for _ in 0..3 {
                    print!("\x07");
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            _ => {}
        }
    }
}

pub struct NoSoundSystem;

impl SoundSystem for NoSoundSystem {
    fn play_sound(&self, _event: GameEvent) {
        // Do nothing
    }
}
