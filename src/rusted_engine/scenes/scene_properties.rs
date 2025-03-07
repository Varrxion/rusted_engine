use nalgebra::Vector2;

#[derive(Clone)]
pub struct SceneProperties {
    gravity: Vector2<f32>,
    terminal_velocity: Vector2<f32>,
}

impl SceneProperties {
    pub fn new(gravity: Vector2<f32>, terminal_velocity: Vector2<f32>) -> Self {
        SceneProperties {
            gravity,
            terminal_velocity,
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