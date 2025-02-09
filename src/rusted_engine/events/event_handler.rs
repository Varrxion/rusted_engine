use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use rusted_open::framework::graphics::util::master_graphics_list::MasterGraphicsList;

use crate::rusted_engine::{audio::audio_manager::{AudioManager, AudioType}, entities::util::master_entity_list::MasterEntityList};

use super::collision::{self};

pub struct EventHandler {
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    audio_manager: Arc<RwLock<AudioManager>>,
}

impl EventHandler {
    pub fn new(
        master_entity_list: Arc<RwLock<MasterEntityList>>,
        master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
        audio_manager: Arc<RwLock<AudioManager>>,
    ) -> Self {
        Self {
            master_entity_list,
            master_graphics_list,
            audio_manager,
        }
    }

    pub fn process_collisions(&self) {
        let collision_events = collision::check_active_entity_collisions(self.master_entity_list.clone(), self.master_graphics_list.clone());
        collision::handle_collision_events(collision_events, &self.master_entity_list.write().unwrap(), &self.master_graphics_list.write().unwrap(), &self.audio_manager.read().unwrap());
    }

    pub fn destroy_object(&self, object_name: String) {
        self.master_entity_list.write().unwrap().remove_entity(&object_name);
        self.master_graphics_list.write().unwrap().remove_object(&object_name);
    }

    pub fn homebringer_sequence(&self) {
        if let Some(player_object) = self.master_graphics_list.read().unwrap().get_object("testscene_playersquare") {
            player_object.write().unwrap().set_position(Vector3::new(0.0, 0.0, 0.0));
            self.audio_manager.read().unwrap().enqueue_audio("Homebringer", AudioType::UI, 0.6, false);
        }
    }

    pub fn gorbino_sequence(&self) {
        self.audio_manager.read().unwrap().enqueue_audio("gorbino", AudioType::Music, 0.6, false);
    }

    pub fn explosion_sequence(&self) {
        if let Some(player_object) = self.master_graphics_list.read().unwrap().get_object("testscene_obj4") {
            player_object.write().unwrap().set_position(Vector3::new(0.0, 0.0, 0.0));
            self.audio_manager.read().unwrap().enqueue_audio("RobloxExplosion", AudioType::UI, 0.6, false);
        }
    }
}
