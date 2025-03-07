use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use rusted_open::framework::graphics::util::master_graphics_list::MasterGraphicsList;

use crate::rusted_engine::{audio::audio_manager::{AudioManager, AudioType}, entities::util::master_entity_list::MasterEntityList, game_state::GameState, scenes::scene_manager::SceneManager};

use super::{collision::{self, resolve_overlap, transfer_velocity_on_collision, CollisionEvent}, triggers::{Outcome, Trigger, TriggerConditions, TriggerType}};

pub struct EventHandler {
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    audio_manager: Arc<RwLock<AudioManager>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    game_state: Arc<RwLock<GameState>>,
    event_outcomes: Vec<Outcome>,
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
            event_outcomes: Vec::new(),
        }
    }

    pub fn process_event_outcomes(&mut self) {
        let mut index = 0;
    
        while index < self.event_outcomes.len() {
            let event_outcome = &self.event_outcomes[index];
            
            match event_outcome {
                Outcome::Sequence(sequence_args) => {
                    match sequence_args.sequence_name.as_str() {
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
                        _ => {
                            println!("No sequence found for sequence name: {}", sequence_args.sequence_name);
                        }
                    }
                }
                Outcome::SwapScene(swap_scene_args) => {
                    if !swap_scene_args.scene_name.is_empty() {
                        self.swap_scene(swap_scene_args.scene_name.clone());
                    }
                }
                Outcome::DestroyObject(destroy_args) => {
                    if !destroy_args.object_name.is_empty() {
                        let chained_outcomes = self.destroy_object(destroy_args.object_name.clone());
                        self.event_outcomes.extend(chained_outcomes);
                    }
                }
                Outcome::EnqueueAudio(audio_args) => {
                    if !audio_args.audio_name.is_empty() {
                        self.enqueue_audio(audio_args.audio_name.clone(), audio_args.audio_type.clone(), audio_args.volume);
                    }
                }
                _ => {
                    println!("Unhandled outcome: {:?}", event_outcome);
                }
            }
    
            index += 1;
        }
    
        self.event_outcomes.clear();
    }
    

    pub fn process_collisions(&mut self) {
        let collision_events = collision::check_active_entity_collisions(self.master_entity_list.clone(), self.master_graphics_list.clone());
        let mut event_outcomes = self.handle_collision_events(collision_events);
        self.event_outcomes.append(&mut event_outcomes);
    }

    pub fn handle_collision_events(&mut self, collision_events: Vec<CollisionEvent>) -> Vec<Outcome> {
        let mut event_outcomes: Vec<Outcome> = Vec::new();

        let master_entity_list = self.master_entity_list.read().unwrap();
        let master_graphics_list = self.master_graphics_list.read().unwrap();
        for collision_event in collision_events {
            if let Some(entity_1) = master_entity_list.get_entity(&collision_event.object_name_1) {
                if let Ok(mut entity_1) = entity_1.write() {
                    if let Some(entity_2) = master_entity_list.get_entity(&collision_event.object_name_2) {
                        if let Ok(mut entity_2) = entity_2.write() {
                            // Check collision triggers before we might destroy the object.
                            self.check_collision_triggers(entity_1.get_triggers(), entity_2.get_name().to_owned(), &mut event_outcomes);
                            self.check_collision_triggers(entity_2.get_triggers(), entity_1.get_name().to_owned(), &mut event_outcomes);
    
                            // Resolve overlap first
                            resolve_overlap(&mut entity_1, &mut entity_2, &master_graphics_list);
    
                            // Transfer velocities based on the collision and weights
                            transfer_velocity_on_collision(&mut entity_1, &mut entity_2);
                        }
                    }
                }
            }
        }
        return event_outcomes
    }

    fn check_collision_triggers(&self, triggers: &Vec<Trigger>, entity_2_name: String, event_outcomes: &mut Vec<Outcome>) {
        for trigger in triggers {
            if let TriggerType::Collision = trigger.trigger_type {
                if let Some(TriggerConditions::CollisionConditions(cond)) = &trigger.conditions {
                    if cond.collided_with == entity_2_name {
                        if let Some(outcome) = &trigger.outcome {
                            event_outcomes.push(outcome.clone());
                        }
                    }
                } else {
                    if let Some(outcome) = &trigger.outcome {
                        event_outcomes.push(outcome.clone());
                    }
                }
            }
        }
    }
    
    

    fn check_destruction_triggers(&self, triggers: &Vec<Trigger>, event_outcomes: &mut Vec<Outcome>) {
        for trigger in triggers {
            if let TriggerType::Destruction = trigger.trigger_type {
                if let Some(outcome) = &trigger.outcome {
                    event_outcomes.push(outcome.clone());
                }
            }
        }
    }

    pub fn swap_scene(&self, scene_name: String) {
        self.master_entity_list.write().unwrap().remove_all();
        self.master_graphics_list.write().unwrap().remove_all();
        self.scene_manager.read().unwrap().load_scene(&mut self.game_state.write().unwrap(), &self.master_entity_list.write().unwrap(), &self.master_graphics_list.write().unwrap(), scene_name);
    }
    
    pub fn destroy_object(&self, entity_name: String) -> Vec<Outcome> {
        let mut event_outcomes: Vec<Outcome> = Vec::new();

        if let Some(entity) = self.master_entity_list.read().unwrap().get_entity(&entity_name) {
            if let Ok(entity) = entity.read() {
                self.check_destruction_triggers(entity.get_triggers(), &mut event_outcomes);
            }
        }

        self.master_entity_list.write().unwrap().remove_entity(&entity_name);
        self.master_graphics_list.write().unwrap().remove_object(&entity_name);

        return event_outcomes;
    }

    pub fn enqueue_audio(&self, audio_name: String, audio_type: AudioType, volume: f32) {
        self.audio_manager.read().unwrap().enqueue_audio(&audio_name, audio_type, volume, false);
    }

    pub fn homebringer_sequence(&self) {
        if let Some(player_object) = self.master_graphics_list.read().unwrap().get_object("player") {
            let player_z = player_object.read().unwrap().get_position().z;
            player_object.write().unwrap().set_position(Vector3::new(0.0, 0.0, player_z));
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
        if let Some(player_entity) = self.master_entity_list.read().unwrap().get_entity("player") {
            let mut player_entity_write = player_entity.write().unwrap();
            let toggle_gravity = !player_entity_write.is_affected_by_gravity();
            player_entity_write.set_affected_by_gravity(toggle_gravity);
            self.audio_manager.read().unwrap().enqueue_audio("Gravity", AudioType::UI, 0.6, false);
        }
    }

    pub fn reset_sequence(&self) {
        self.swap_scene("testscene".to_owned());
        self.audio_manager.read().unwrap().enqueue_audio("TechMysterious", AudioType::UI, 0.6, false);
    }
}
