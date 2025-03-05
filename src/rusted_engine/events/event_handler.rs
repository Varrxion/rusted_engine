use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use rusted_open::framework::graphics::util::master_graphics_list::MasterGraphicsList;

use crate::rusted_engine::{audio::audio_manager::{AudioManager, AudioType}, entities::util::master_entity_list::MasterEntityList, game_state::GameState, scenes::scene_manager::SceneManager};

use super::{collision::{self, resolve_overlap, transfer_velocity_on_collision, CollisionEvent}, triggers::{Trigger, TriggerConditions, TriggerType}};

pub struct EventOutcome {
    outcome: String,
    target: String,
}

pub struct EventHandler {
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    audio_manager: Arc<RwLock<AudioManager>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    game_state: Arc<RwLock<GameState>>,
    event_outcomes: Vec<EventOutcome>,
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
            let outcome = &event_outcome.outcome;
            let target_name = &event_outcome.target;
    
            match outcome.as_str() {
                "swap_scene" => {
                    if target_name != "" {
                        self.swap_scene(target_name.to_string());
                    }
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
                        let chained_outcomes = self.destroy_object(target_name.to_string());
                        self.event_outcomes.extend(chained_outcomes);
                    }
                }
                _ => {
                    println!("No outcome found for outcome: {}", outcome);
                }
            }
    
            index += 1;
        }
        self.event_outcomes.clear();
    }
    

    pub fn swap_scene(&self, scene_name: String) {
        self.master_entity_list.write().unwrap().remove_all();
        self.master_graphics_list.write().unwrap().remove_all();
        self.scene_manager.read().unwrap().load_scene(&mut self.game_state.write().unwrap(), &self.master_entity_list.write().unwrap(), &self.master_graphics_list.write().unwrap(), scene_name);
    }

    pub fn process_collisions(&mut self) {
        let collision_events = collision::check_active_entity_collisions(self.master_entity_list.clone(), self.master_graphics_list.clone());
        let mut event_outcomes = self.handle_collision_events(collision_events);
        self.event_outcomes.append(&mut event_outcomes);
    }

    pub fn handle_collision_events(&mut self, collision_events: Vec<CollisionEvent>) -> Vec<EventOutcome> {
        let mut event_outcomes: Vec<EventOutcome> = Vec::new();

        let master_entity_list = self.master_entity_list.read().unwrap();
        let master_graphics_list = self.master_graphics_list.read().unwrap();
        let audio_manager = self.audio_manager.write().unwrap();
        for collision_event in collision_events {
            if let Some(entity_1) = master_entity_list.get_entity(&collision_event.object_name_1) {
                if let Ok(mut entity_1) = entity_1.write() {
                    if let Some(entity_2) = master_entity_list.get_entity(&collision_event.object_name_2) {
                        if let Ok(mut entity_2) = entity_2.write() {
                            // Check collision triggers before we might destroy the object.
                            self.check_collision_triggers(entity_1.get_triggers(), entity_2.get_name().to_owned(), &mut event_outcomes);
                            self.check_collision_triggers(entity_2.get_triggers(), entity_1.get_name().to_owned(), &mut event_outcomes);

                            // Handle destruction logic based on weight
                            if entity_1.can_destroy() && entity_2.is_destructible() && entity_1.get_weight() >= entity_2.get_weight() {
                                let event_outcome = EventOutcome {
                                    outcome: "destroy_object".to_owned(),
                                    target: entity_2.get_name().to_owned(),
                                };
                                event_outcomes.push(event_outcome);
                            }
                            if entity_2.can_destroy() && entity_1.is_destructible() && entity_2.get_weight() >= entity_1.get_weight() {
                                let event_outcome = EventOutcome {
                                    outcome: "destroy_object".to_owned(),
                                    target: entity_1.get_name().to_owned(),
                                };
                                event_outcomes.push(event_outcome);
                            }
    
                            // Resolve overlap first
                            resolve_overlap(&mut entity_1, &mut entity_2, &master_graphics_list);
    
                            // Transfer velocities based on the collision and weights
                            transfer_velocity_on_collision(&mut entity_1, &mut entity_2);
    
                            // Handle sound effects
                            let entity_1_collision_sound = entity_1.get_collision_sound();
                            let entity_2_collision_sound = entity_2.get_collision_sound();
                            if entity_2_collision_sound != "" && entity_2_collision_sound != "null" && entity_2_collision_sound != "none" {
                                audio_manager.enqueue_audio(entity_2_collision_sound, AudioType::Sound, 0.4, false);
                            } else if entity_1_collision_sound != "" && entity_1_collision_sound != "null" && entity_1_collision_sound != "none" {
                                audio_manager.enqueue_audio(entity_1_collision_sound, AudioType::Sound, 0.4, false);
                            }

                            self.check_collision_triggers(entity_1.get_triggers(), entity_2.get_name().to_owned(), &mut event_outcomes);
                            self.check_collision_triggers(entity_2.get_triggers(), entity_1.get_name().to_owned(), &mut event_outcomes);
                        }
                    }
                }
            }
        }
        return event_outcomes
    }

    fn check_collision_triggers(&self, triggers: &Vec<Trigger>, entity_2_name: String, event_outcomes: &mut Vec<EventOutcome>) {
        for trigger in triggers {
            if let TriggerType::Collision = trigger.trigger_type {
                if let TriggerConditions::CollisionConditions(cond) = &trigger.conditions {
                    if cond.object_name == entity_2_name {
                        if let Some(outcome) = &trigger.outcome {
                            if let Some(target) = &trigger.target {
                                let event_outcome = EventOutcome {
                                    outcome: outcome.to_string(),
                                    target: target.to_string(),
                                };
                                event_outcomes.push(event_outcome);
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_destruction_triggers(&self, triggers: &Vec<Trigger>, event_outcomes: &mut Vec<EventOutcome>) {
        for trigger in triggers {
            if let TriggerType::Destruction = trigger.trigger_type {
                if let Some(outcome) = &trigger.outcome {
                    if let Some(target) = &trigger.target {
                        let event_outcome = EventOutcome {
                            outcome: outcome.to_string(),
                            target: target.to_string(),
                        };
                        event_outcomes.push(event_outcome);
                    }
                }
            }
        }
    }
    
    pub fn destroy_object(&self, entity_name: String) -> Vec<EventOutcome> {
        let mut event_outcomes: Vec<EventOutcome> = Vec::new();

        if let Some(entity) = self.master_entity_list.read().unwrap().get_entity(&entity_name) {
            if let Ok(entity) = entity.read() {
                self.check_destruction_triggers(entity.get_triggers(), &mut event_outcomes);
            }
        }

        self.master_entity_list.write().unwrap().remove_entity(&entity_name);
        self.master_graphics_list.write().unwrap().remove_object(&entity_name);

        return event_outcomes;
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

    pub fn reset_sequence(&self) {
        self.swap_scene("testscene".to_owned());
        self.audio_manager.read().unwrap().enqueue_audio("TechMysterious", AudioType::UI, 0.6, false);
    }
}
