use nalgebra::Vector2;

pub struct GameState {
    gravity: Vector2<f32>,
    terminal_velocity: Vector2<f32>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            gravity: Vector2::new(0.0, 0.0),
            terminal_velocity: Vector2::new(f32::MAX, f32::MAX),
        }
    }

    pub fn set_gravity(&mut self, gravity: Vector2<f32>) {
        self.gravity = gravity;
    }

    pub fn get_gravity(&self) -> Vector2<f32> {
        self.gravity
    }

    pub fn set_terminal_velocity(&mut self, terminal_velocity: Vector2<f32>) {
        self.terminal_velocity = terminal_velocity;
    }

    pub fn get_terminal_velocity(&self) -> Vector2<f32> {
        self.terminal_velocity
    }
}
