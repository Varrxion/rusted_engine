use std::time::Instant;

pub struct MasterClock {
    last_time: Instant,
    delta_time: f32,
    max_delta: f32, // Maximum allowed delta time, for preventing clips at low framerates
}

impl MasterClock {
    /// Creates a new MasterClock instance.
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            delta_time: 0.0,
            max_delta: 0.1,
        }
    }

    /// Updates the clock by calculating the delta time since the last update.
    pub fn update(&mut self) {
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(self.last_time).as_secs_f32();

        self.delta_time = elapsed.min(self.max_delta);
        self.last_time = current_time;
    }

    /// Returns the time elapsed since the last update.
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}
