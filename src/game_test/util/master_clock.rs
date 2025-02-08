use std::time::Instant;

pub struct MasterClock {
    last_time: Instant,
    delta_time: f32,
}

impl MasterClock {
    /// Creates a new MasterClock instance.
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            delta_time: 0.0,
        }
    }

    /// Updates the clock by calculating the delta time since the last update.
    pub fn update(&mut self) {
        let current_time = Instant::now();
        self.delta_time = current_time.duration_since(self.last_time).as_secs_f32();
        self.last_time = current_time;
    }

    /// Returns the time elapsed since the last update.
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}
