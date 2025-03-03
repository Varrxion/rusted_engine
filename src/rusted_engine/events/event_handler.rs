use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use rusted_open::framework::graphics::util::master_graphics_list::MasterGraphicsList;

use crate::rusted_engine::{audio::audio_manager::{AudioManager, AudioType}, entities::util::master_entity_list::MasterEntityList, game_state::GameState, scenes::scene_manager::SceneManager};

use super::collision::{self};

pub struct EventHandler {
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    audio_manager: Arc<RwLock<AudioManager>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    game_state: Arc<RwLock<GameState>>,
}

impl EventHandler {
    pub fn new(
        master_entity_list: Arc<RwLock<MasterEntityList>>,
        master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
        audio_manager: Arc<RwLock<AudioManager>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        game_state: Arc<RwLock<GameState>>,
    ) -> Self {
        Self {
            master_entity_list,
            master_graphics_list,
            audio_manager,
            scene_manager,
            game_state,
        }
    }

    pub fn interpret_trigger(&self, trigger_name: String, target_name: String) {
        match trigger_name.as_str() {
            "swap_scene" => {
                // You might want to pass a scene name dynamically
                self.swap_scene("default_scene".to_string());
            }
            "homebringer_sequence" => {
                self.homebringer_sequence();
            }
            "gorbino_sequence" => {
                self.gorbino_sequence();
            }
            "explosion_sequence" => {
                self.explosion_sequence();
            }
            "gravity_sequence" => {
                self.gravity_sequence();
            }
            "destroy_object" => {
                if target_name != "" {
                    self.destroy_object(target_name);
                }
            }
            _ => {
                println!("No trigger found for {}", trigger_name);
            }
        }
    }
    

    pub fn swap_scene(&self, scene_name: String) {
        self.master_entity_list.write().unwrap().remove_all();
        self.master_graphics_list.write().unwrap().remove_all();
        self.scene_manager.read().unwrap().load_scene(&mut self.game_state.write().unwrap(), &self.master_entity_list.read().unwrap(), &self.master_graphics_list.read().unwrap(), scene_name);
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

    pub fn gravity_sequence(&self) {
        if let Some(player_entity) = self.master_entity_list.read().unwrap().get_entity("testscene_playersquare") {
            let mut player_entity_write = player_entity.write().unwrap();
            let toggle_gravity = !player_entity_write.is_affected_by_gravity();
            player_entity_write.set_affected_by_gravity(toggle_gravity);
            self.audio_manager.read().unwrap().enqueue_audio("Gravity", AudioType::UI, 0.6, false);
        }
    }
}
