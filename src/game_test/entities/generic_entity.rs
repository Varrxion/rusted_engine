pub struct GenericEntity {
    object_name: String, // the name of the object so we can interact with it on the master graphics list
    weight: f32, // In collision disputes the object with the higher weight can be the one that does not get moved
    can_destroy: bool, // When colliding with an object of lower weight, if that object is destructible should we destroy it?
    destructible: bool, // When colliding with an object of higher weight, if that object can destroy, should we be destroyed?
    active_collision: bool, // Whether this object should be checked actively as the source of a collision, or passively (can be collided with by active objects)
}

impl GenericEntity {
    pub fn new(object_name: String, weight: f32, can_destroy: bool, destructible: bool, active_collision: bool) -> Self {
        GenericEntity {
            object_name,
            weight,
            can_destroy,
            destructible,
            active_collision,
        }
    }


}