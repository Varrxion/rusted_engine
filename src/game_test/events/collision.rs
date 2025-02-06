use rusted_open::engine::{events::collision, graphics::util::master_graphics_list::MasterGraphicsList};

pub struct Collision;

impl Collision {
    /// Wrapper for the check_collisions function.
    pub fn check_collisions(master_graphics_list: &MasterGraphicsList, object_name: &str) -> Vec<collision::CollisionEvent> {
        collision::check_collisions(master_graphics_list, object_name)
    }
}