use std::sync::{Arc, RwLock};

use glfw::Key;
use nalgebra::Vector3;
use rusted_open::engine::{engine_controller::EngineController, events::movement, graphics::{internal_object::graphics_object::Generic2DGraphicsObject, texture_manager::TextureManager, util::{master_clock::MasterClock, master_graphics_list::MasterGraphicsList}}, input::key_states::KeyStates};

use super::{audio::audio_manager::{AudioManager, AudioType}, entities::{generic_entity::GenericEntity, util::master_entity_list::MasterEntityList}, events::event_handler::EventHandler, scenes::scene_manager::SceneManager};

pub struct GameTestController {
    engine_controller: EngineController,
    event_handler: EventHandler,
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    audio_manager: Arc<RwLock<AudioManager>>,
}

impl GameTestController {
    /// Creates a new EntryPoint instance.
    pub fn new() -> Self {
        let window_name = "Game Test";
        GameTestController {
            engine_controller: EngineController::new(window_name),
            event_handler: EventHandler,
            master_entity_list: Arc::new(RwLock::new(MasterEntityList::new())),
            scene_manager: Arc::new(RwLock::new(SceneManager::new())),
            audio_manager: Arc::new(RwLock::new(AudioManager::new())),
        }
    }

    // Call from main to start everything
    pub fn init(&mut self) {
        // Grab the parts of the engine_controller we want to use
        let master_clock = self.engine_controller.get_master_clock();
        let texture_manager = self.engine_controller.get_texture_manager();
        let master_graphics_list = self.engine_controller.get_master_graphics_list();
        let key_states = self.engine_controller.get_key_states();

        // Go into this function to see how the loading is done.
        self.load_resources(&texture_manager.write().unwrap(), &master_graphics_list.write().unwrap());

        // Test some music
        //audio_manager.write().unwrap().enqueue_audio("RealiteVirtuelle", AudioType::Music, 0.2, true);

        let mut flag = false;

        while flag == false {
            flag = self.main_loop(&master_clock, &master_graphics_list, &key_states);
        }
    }

    /// This is the main loop for the framework.
    /// I have included a simple control scheme for the object we control, a random spinning object, and two stationary objects.
    pub fn main_loop(&mut self, master_clock: &Arc<RwLock<MasterClock>>, master_graphics_list: &Arc<RwLock<MasterGraphicsList>>, key_states: &Arc<RwLock<KeyStates>>) -> bool {
        // Retrieve the "player" square from the master graphics list
        let square = master_graphics_list.read().unwrap().get_object("testscene_playersquare").expect("Object not found");

        // Thou shalt not use frame-based physics.
        let delta_time = master_clock.read().unwrap().get_delta_time();

        // Just here for now, do movement inputs
        self.process_player_movement(key_states, square, delta_time);

        // Process audio inputs
        self.process_piano_keys(key_states, &self.audio_manager);

        // Spin this object for testing
        if let Some(object_2) = master_graphics_list.read().unwrap().get_object("testscene_obj1") {
            let mut object_2_read = object_2.write().unwrap();
            let rotfactor = object_2_read.get_rotation()+1.0*delta_time;
            object_2_read.set_rotation(rotfactor);
        } else {
            println!("No object found with name testscene_obj1.");
        }

        // Call the collision checking method
        let collision_events = self.event_handler.check_entity_collisions(&self.master_entity_list.read().unwrap(), &master_graphics_list.read().unwrap());
        self.event_handler.handle_collision_events(&self.master_entity_list.read().unwrap(), &master_graphics_list.read().unwrap(), collision_events);

        let _ = self.audio_manager.write().unwrap().process_audio_queue();

        master_graphics_list.read().unwrap().debug_all();

        return self.engine_controller.execute_tick();

    }

    /// Here we will load the json scene configs (basically level files), and load the test scene into the master graphics list.
    pub fn load_resources(&mut self, texture_manager: &TextureManager, master_graphics_list: &MasterGraphicsList) {
        self.engine_controller.set_resolution(1280.0, 720.0);
        let mut scene_manager = self.scene_manager.write().unwrap();
        let audio_manager = self.audio_manager.read().unwrap();

        // Load the texture files and the scenes from their respective directories into memory
        let _ = texture_manager.load_textures_from_directory("src\\resources\\textures");
        let _ = texture_manager.load_textures_from_directory("src\\resources\\localonly\\textures");
        let _ = scene_manager.load_scenes_from_directory("src\\resources\\scenes", &texture_manager);
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\sounds");
        // Load resources which should not be uploaded
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\localonly\\music");
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\localonly\\sounds");

        // Load the test scene from the manager into the master graphics list
        let scene_name = "testscene";
        scene_manager.load_scene_into_master_graphics_list(master_graphics_list, scene_name.to_owned());

        let test_entity_1 = Arc::new(RwLock::new(GenericEntity::new("testscene_playersquare".to_owned(), 3.0, true, false, true)));
        let test_entity_2 = Arc::new(RwLock::new(GenericEntity::new("testscene_obj2".to_owned(), 2.0, false, false, false)));
        let test_entity_3 = Arc::new(RwLock::new(GenericEntity::new("testscene_obj3".to_owned(), 3.0, false, false, false)));
        let test_entity_4 = Arc::new(RwLock::new(GenericEntity::new("testscene_obj4".to_owned(), 4.0, false, false, false)));
        
        
        self.master_entity_list.write().unwrap().add_entity(test_entity_1);
        self.master_entity_list.write().unwrap().add_entity(test_entity_2);
        self.master_entity_list.write().unwrap().add_entity(test_entity_3);
        self.master_entity_list.write().unwrap().add_entity(test_entity_4);
    }

    // Apply movement based on active keys
    pub fn process_player_movement(&self, key_states: &Arc<RwLock<KeyStates>>, square: Arc<RwLock<Generic2DGraphicsObject>>, delta_time: f32) {
        let move_speed = 0.2;
        let rotation_speed = 2.0;
        let key_states_read = key_states.read().unwrap();

        if key_states_read.is_key_pressed_raw(Key::W) {
            movement::move_object(square.clone(), Vector3::new(0.0, 1.0, 0.0), move_speed, delta_time);
        }
        if key_states_read.is_key_pressed_raw(Key::S) {
            movement::move_object(square.clone(), Vector3::new(0.0, -1.0, 0.0), move_speed, delta_time);
        }
        if key_states_read.is_key_pressed_raw(Key::A) {
            movement::move_object(square.clone(), Vector3::new(-1.0, 0.0, 0.0), move_speed, delta_time);
        }
        if key_states_read.is_key_pressed_raw(Key::D) {
            movement::move_object(square.clone(), Vector3::new(1.0, 0.0, 0.0), move_speed, delta_time);
        }
        if key_states_read.is_key_pressed_raw(Key::Q) {
            movement::rotate_object(square.clone(), rotation_speed*delta_time);
        }
        if key_states_read.is_key_pressed_raw(Key::E) {
            movement::rotate_object(square.clone(), -rotation_speed*delta_time);
        }
    }

    // Here we will check if the numpad keys are pressed and it will make different piano sounds. For testing.
    pub fn process_piano_keys(&self, key_states: &Arc<RwLock<KeyStates>>, audio_manager: &Arc<RwLock<AudioManager>>) {
        let audio_manager_write = audio_manager.write().unwrap();
        let key_states_read = key_states.read().unwrap();

        if key_states_read.is_key_pressed(Key::Kp0) {
            audio_manager_write.stop_audio();
        }
        if key_states_read.is_key_pressed(Key::Kp1) {
            audio_manager_write.enqueue_audio("Piano4A", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp2) {
            audio_manager_write.enqueue_audio("Piano4B", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp3) {
            audio_manager_write.enqueue_audio("Piano5C", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp4) {
            audio_manager_write.enqueue_audio("Piano5D", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp5) {
            audio_manager_write.enqueue_audio("gorbino", AudioType::Music, 0.8, false);
        }
        if key_states_read.is_key_pressed(Key::Kp6) {
            audio_manager_write.enqueue_audio("Piano5E", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp7) {
            audio_manager_write.enqueue_audio("Piano5F", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp8) {
            audio_manager_write.enqueue_audio("Piano5G", AudioType::Sound, 1.0, false);
        }
        if key_states_read.is_key_pressed(Key::Kp9) {
            audio_manager_write.enqueue_audio("Piano5A", AudioType::Sound, 1.0, false);
        }
    }
}