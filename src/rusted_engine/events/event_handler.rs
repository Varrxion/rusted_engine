use std::{collections::HashSet, sync::{Arc, RwLock}};

use nalgebra::{Vector2, Vector3};
use rusted_open::framework::graphics::{internal_object::{animation_config::AnimationConfig, atlas_config::AtlasConfig, custom_shader::CustomShader, graphics_object::Generic2DGraphicsObject}, texture_manager::TextureManager, util::master_graphics_list::MasterGraphicsList};

use crate::rusted_engine::{audio::audio_manager::{AudioManager, AudioType}, entities::{generic_entity::{CollisionMode, GenericEntity}, util::master_entity_list::MasterEntityList}, game_state::GameState, input::key_states::KeyStates, scenes::scene_manager::{ObjectData, SceneManager}, util::char_to_glfw_key::char_to_glfw_key};

use super::{collision::{self, resolve_overlap, transfer_velocity_on_collision, CollisionEvent}, triggers::{KeyCondition, Outcome, SceneTriggerType, Trigger, TriggerConditions, TriggerType}};

pub struct EventHandler {
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    texture_manager: Arc<RwLock<TextureManager>>,
    audio_manager: Arc<RwLock<AudioManager>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    game_state: Arc<RwLock<GameState>>,
    event_outcomes: Vec<Outcome>,
    key_states: Arc<RwLock<KeyStates>>,
}

impl EventHandler {
    pub fn new(
        master_entity_list: Arc<RwLock<MasterEntityList>>,
        master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
        texture_manager: Arc<RwLock<TextureManager>>,
        audio_manager: Arc<RwLock<AudioManager>>,
        scene_manager: Arc<RwLock<SceneManager>>,
        game_state: Arc<RwLock<GameState>>,
        key_states: Arc<RwLock<KeyStates>>,
    ) -> Self {
        Self {
            master_entity_list,
            master_graphics_list,
            texture_manager,
            audio_manager,
            scene_manager,
            game_state,
            key_states,
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
                Outcome::CreateObject(create_object_args) => {
                    if self.master_graphics_list.read().unwrap().get_object(&create_object_args.graphics.name).is_some() {
                        println!("Call to create object was made when object of same ID \"{}\" already exists. Ignoring.", create_object_args.graphics.name)
                    } else {
                        self.create_object(create_object_args.clone());
                    }
                }
                Outcome::DestroyObject(destroy_args) => {
                    if !destroy_args.object_name.is_empty() {
                        let chained_outcomes = self.destroy_object(destroy_args.object_name.clone());
                        self.event_outcomes.extend(chained_outcomes);
                    }
                }
                Outcome::TeleportObject(teleport_args) => {
                    if !teleport_args.object_name.is_empty() {
                        self.teleport_object(teleport_args.object_name.clone(), teleport_args.new_position.clone());
                    }
                }
                Outcome::EnqueueAudio(audio_args) => {
                    if !audio_args.audio_name.is_empty() {
                        self.enqueue_audio(audio_args.audio_name.clone(), audio_args.audio_type.clone(), audio_args.volume);
                    }
                }
                Outcome::SetAtlasConfig(set_atlas_config_args) => {
                    if !set_atlas_config_args.object_name.is_empty() {
                        self.set_atlas_config(set_atlas_config_args.object_name.clone(), set_atlas_config_args.atlas_config.clone());
                    }
                }
                Outcome::SetAnimationConfig(set_animation_config_args) => {
                    if !set_animation_config_args.object_name.is_empty() {
                        self.set_animation_config(set_animation_config_args.object_name.clone(), set_animation_config_args.animation_config.clone());
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
                        event_outcomes.push(trigger.outcome.clone());
                    }
                } else {
                    event_outcomes.push(trigger.outcome.clone());
                }
            }
        }
    }

    fn check_destruction_triggers(&self, triggers: &Vec<Trigger>, event_outcomes: &mut Vec<Outcome>) {
        for trigger in triggers {
            if let TriggerType::Destruction = trigger.trigger_type {
                event_outcomes.push(trigger.outcome.clone());
            }
        }
    }

    pub fn check_scene_triggers(&mut self) {
        let current_scene_name = self.game_state.read().unwrap().get_current_scene_name();

        // This if statement should never fail
        if let Some(current_scene) = self.scene_manager.read().unwrap().get_scene(&current_scene_name) {
            let current_scene_triggers = current_scene.read().unwrap().get_triggers();

            for scene_trigger in current_scene_triggers {
                match scene_trigger.scene_trigger_type {
                    SceneTriggerType::KeyPressed => {
                        if let Some(TriggerConditions::KeyConditions(cond)) = &scene_trigger.conditions {
                            if self.check_key_pressed_trigger(cond.clone()) {
                                for outcome in scene_trigger.outcome {
                                    self.event_outcomes.push(outcome)
                                }
                            }
                        } else {
                            println!("A KeyPressed trigger was processed, but no condition could be found. Ignoring.");
                        }
                    }
                    SceneTriggerType::KeyNotPressed => {
                        if let Some(TriggerConditions::KeyConditions(cond)) = &scene_trigger.conditions {
                            if self.check_key_not_pressed_trigger(cond.clone()) {
                                for outcome in scene_trigger.outcome {
                                    self.event_outcomes.push(outcome)
                                }
                            }
                        } else {
                            println!("A KeyNotPressed trigger was processed, but no condition could be found. Ignoring.");
                        }
                    }
                    SceneTriggerType::Timer => {
                        self.check_timer_trigger();
                    }
                    _ => {
                        println!("The scene trigger was empty.")
                    }
                }
            }
        }
    }

    pub fn check_key_pressed_trigger(&self, trigger_condition: KeyCondition) -> bool {
        for key in trigger_condition.keys.iter() {
            if let Some(key) = char_to_glfw_key(*key) {
                if self.key_states.read().unwrap().is_key_pressed_raw(key) {
                    return true;
                }
            }
        }
        return false;
    }
    

    pub fn check_key_not_pressed_trigger(&self, trigger_condition: KeyCondition) -> bool {
        for key in trigger_condition.keys.iter() {
            if let Some(key) = char_to_glfw_key(*key) {
                if self.key_states.read().unwrap().is_key_pressed_raw(key) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn check_timer_trigger(&self) {
        println!("Timer trigger is not implemented yet");
    }

    pub fn swap_scene_without_saving(&self, scene_name: String) {
        self.master_entity_list.write().unwrap().remove_all();
        self.master_graphics_list.write().unwrap().remove_all();
        self.scene_manager.read().unwrap().load_scene(&mut self.game_state.write().unwrap(), &self.master_entity_list.write().unwrap(), &self.master_graphics_list.write().unwrap(), scene_name);
    }

    pub fn swap_scene(&self, scene_name: String) {
        self.scene_manager.write().unwrap().save_scene(&self.game_state.read().unwrap().get_current_scene_name(), &self.master_entity_list.read().unwrap(), &self.master_graphics_list.read().unwrap());

        self.master_entity_list.write().unwrap().remove_all();
        self.master_graphics_list.write().unwrap().remove_all();
        self.scene_manager.read().unwrap().load_scene(&mut self.game_state.write().unwrap(), &self.master_entity_list.write().unwrap(), &self.master_graphics_list.write().unwrap(), scene_name);
    }

    pub fn create_object(&self, create_object_args: ObjectData) {

        let json_shader = CustomShader::new(
            &create_object_args.graphics.vertex_shader,
            &create_object_args.graphics.fragment_shader,
        );

        // Handle optional AnimationConfig
        let animation_config = create_object_args.graphics.animation_config.map(|animation_config| AnimationConfig {
            looping: animation_config.looping,
            mode: animation_config.mode.clone(),
            frame_range: animation_config.frame_range.clone(),
            frame_duration: animation_config.frame_duration,
        });

        // Handle optional AtlasConfig
        let atlas_config = create_object_args.graphics.atlas_config.map(|atlas_config| AtlasConfig {
            current_frame: atlas_config.current_frame,
            atlas_columns: atlas_config.atlas_columns,
            atlas_rows: atlas_config.atlas_rows,
        });

        let mut json_collision_modes = HashSet::new();
        for collision_mode in create_object_args.entity.collision_modes {
            match collision_mode.as_str() {
                "AABB" => { json_collision_modes.insert(CollisionMode::AABB); }
                "Circle" => { json_collision_modes.insert(CollisionMode::Circle); }
                "OBB" => { json_collision_modes.insert(CollisionMode::OBB); }
                _ => {}
            }
        }

        let position = Vector3::new(
            create_object_args.graphics.position[0],
            create_object_args.graphics.position[1],
            create_object_args.graphics.position[2],
        );

        let texture_id = self.texture_manager.read().unwrap().get_texture_id(&create_object_args.graphics.texture_name);

        let graphics_object = Generic2DGraphicsObject::new(
            create_object_args.graphics.name.clone(),
            create_object_args.graphics.vertex_data,
            create_object_args.graphics.texture_coords,
            json_shader.get_shader_program(),
            position,
            create_object_args.graphics.rotation,
            create_object_args.graphics.scale,
            texture_id,
            atlas_config,
            animation_config,
        );

        let velocity = create_object_args.entity.velocity.unwrap_or_else(|| vec![0.0, 0.0]);
        let velocity_vector = Vector2::new(velocity[0], velocity[1]);

        // Default collision_priority to 0 if None
        let collision_priority = create_object_args.entity.collision_priority.unwrap_or(0);

        let triggers = create_object_args.entity.triggers.unwrap_or_default();

        let entity = GenericEntity::new(
            create_object_args.entity.name.clone(),
            create_object_args.entity.weight,
            velocity_vector,
            create_object_args.entity.affected_by_gravity,
            create_object_args.entity.is_static,
            create_object_args.entity.elasticity,
            create_object_args.entity.active_collision,
            collision_priority,
            json_collision_modes,
            triggers,
        );

        let wrapped_graphics_object = Arc::new(RwLock::new(graphics_object));
        let wrapped_entity = Arc::new(RwLock::new(entity));

        self.master_entity_list.write().unwrap().add_entity(wrapped_entity);
        self.master_graphics_list.write().unwrap().add_object(wrapped_graphics_object);
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

    pub fn teleport_object(&self, object_name: String, new_position: Vec<f32>) {
        let teleporting_object_option = self.master_graphics_list.write().unwrap().get_object(&object_name);
        if let Some(teleporting_object) = teleporting_object_option {
            let mut teleporting_object_write = teleporting_object.write().unwrap();

            let current_position = teleporting_object_write.get_position();

            // If the provided destination doesn't include a new Z-value we'll keep the existing one
            let new_position = if new_position.len() == 2 {
                Vector3::new(new_position[0], new_position[1], current_position.z)
            } else if new_position.len() == 3 {
                Vector3::new(new_position[0], new_position[1], new_position[2])
            } else {
                println!("Invalid position size, expected 2D or 2D+Z position.");
                return; // Leave before passing anything
            };

            teleporting_object_write.set_position(new_position);
        }
    }

    pub fn enqueue_audio(&self, audio_name: String, audio_type: AudioType, volume: f32) {
        self.audio_manager.read().unwrap().enqueue_audio(&audio_name, audio_type, volume, false);
    }

    pub fn set_atlas_config(&self, object_name: String, new_atlas_config: AtlasConfig) {
        let master_graphics_list_write = self.master_graphics_list.write().unwrap();
        if let Some(object) = master_graphics_list_write.get_object(&object_name) {
            let mut object_write = object.write().unwrap();
            object_write.set_atlas_config(Some(new_atlas_config));
        }
    }

    pub fn set_animation_config(&self, object_name: String, new_animation_config: AnimationConfig) {
        let master_graphics_list_write = self.master_graphics_list.write().unwrap();
        if let Some(object) = master_graphics_list_write.get_object(&object_name) {
            let mut object_write = object.write().unwrap();
            object_write.set_animation_config(Some(new_animation_config));
        }
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
        self.teleport_object("player".to_owned(), vec![0.0, 0.0, 0.0]);
        self.audio_manager.read().unwrap().enqueue_audio("TechMysterious", AudioType::UI, 0.6, false);
    }
}
