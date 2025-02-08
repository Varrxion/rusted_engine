use std::sync::{Arc, RwLock};

use glfw::{Context, GlfwReceiver, Key, WindowEvent};
use nalgebra::Vector3;
use rusted_open::framework::{framework_controller::FrameworkController, events::movement, graphics::{internal_object::graphics_object::Generic2DGraphicsObject, texture_manager::TextureManager, util::master_graphics_list::MasterGraphicsList}};

use super::{audio::audio_manager::{AudioManager, AudioType}, entities::{generic_entity::GenericEntity, util::master_entity_list::MasterEntityList}, events::event_handler::EventHandler, input::key_states::KeyStates, scenes::scene_manager::SceneManager, util::master_clock::MasterClock};

pub struct EngineController {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    engine_controller: FrameworkController,
    event_handler: EventHandler,
    master_clock: Arc<RwLock<MasterClock>>,
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    scene_manager: Arc<RwLock<SceneManager>>,
    audio_manager: Arc<RwLock<AudioManager>>,
    key_states: Arc<RwLock<KeyStates>>,
}

impl EngineController {
    /// Creates a new EntryPoint instance.
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let window_name = "Game Test";
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(640 as u32, 480 as u32, window_name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Make the window's context current
        window.make_current();

        // Enable key events
        window.set_key_polling(true);

        Self {
            glfw,
            window,
            events,
            engine_controller: FrameworkController::new(),
            event_handler: EventHandler,
            master_clock: Arc::new(RwLock::new(MasterClock::new())),
            master_entity_list: Arc::new(RwLock::new(MasterEntityList::new())),
            scene_manager: Arc::new(RwLock::new(SceneManager::new())),
            audio_manager: Arc::new(RwLock::new(AudioManager::new())),
            key_states: Arc::new(RwLock::new(KeyStates::new())),
        }
    }

    // Call from main to start everything
    pub fn init(&mut self) {
        // Grab the parts of the engine_controller we want to use
        let texture_manager = self.engine_controller.get_texture_manager();
        let master_graphics_list = self.engine_controller.get_master_graphics_list();

        // Go into this function to see how the loading is done.
        self.load_resources(&texture_manager.write().unwrap(), &master_graphics_list.write().unwrap());

        // Test some music
        //audio_manager.write().unwrap().enqueue_audio("RealiteVirtuelle", AudioType::Music, 0.2, true);

        let mut flag = false;

        while flag == false {
            flag = self.main_loop(&master_graphics_list);
        }
    }


    /// This is the main loop for the framework.
    /// I have included a simple control scheme for the object we control, a random spinning object, and two stationary objects.
    pub fn main_loop(&mut self, master_graphics_list: &Arc<RwLock<MasterGraphicsList>>) -> bool {

        self.execute_tick(master_graphics_list);

        // Thou shalt not use frame-based physics.
        let delta_time = self.master_clock.read().unwrap().get_delta_time();

        if self.window.should_close() {
            return true;
        }

        // Retrieve the "player" square from the master graphics list
        let square = master_graphics_list.read().unwrap().get_object("testscene_playersquare").expect("Object not found");

        // Just here for now, do movement inputs
        self.process_player_movement(square, delta_time);

        // Process audio inputs
        self.process_piano_keys(&self.audio_manager);

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

        return false;
    }

    /// Executing a tick renders the frame, updates the master clock, plays all queued audio, and updates inputs.
    pub fn execute_tick(&mut self, master_graphics_list: &Arc<RwLock<MasterGraphicsList>>) {
        // Update the clock
        self.master_clock.write().unwrap().update();

        let _ = self.audio_manager.write().unwrap().process_audio_queue();

        // Only uncomment this line if you want tons of information dumped into the console
        //master_graphics_list.read().unwrap().debug_all();

        // Update Pressed Keys to Held Keys
        self.key_states.write().unwrap().update_pressed_to_held();

        // Handle key events
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                _ => {
                    // Add or remove a key from the list of currently held keys based on the current user input
                    self.key_states.write().unwrap().handle_key_event(event);
                }
            }
        }

        self.engine_controller.render(&mut self.window);
    }

    /// Here we will load the json scene configs (basically level files), and load the test scene into the master graphics list.
    pub fn load_resources(&mut self, texture_manager: &TextureManager, master_graphics_list: &MasterGraphicsList) {
        self.set_resolution(1280.0, 720.0);
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
    pub fn process_player_movement(&self, square: Arc<RwLock<Generic2DGraphicsObject>>, delta_time: f32) {
        let move_speed = 0.2;
        let rotation_speed = 2.0;
        let key_states_read = self.key_states.read().unwrap();

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
    pub fn process_piano_keys(&self, audio_manager: &Arc<RwLock<AudioManager>>) {
        let audio_manager_write = audio_manager.write().unwrap();
        let key_states_read = self.key_states.read().unwrap();

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

    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.window.set_size(width as i32, height as i32);
        self.engine_controller.set_resolution(width, height);
    }
}