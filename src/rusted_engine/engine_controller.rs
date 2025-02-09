use std::sync::{Arc, RwLock};

use glfw::{Context, GlfwReceiver, Key, WindowEvent};
use nalgebra::Vector3;
use rusted_open::framework::{framework_controller::FrameworkController, events::movement, graphics::{internal_object::graphics_object::Generic2DGraphicsObject, texture_manager::TextureManager, util::master_graphics_list::MasterGraphicsList}};

use super::{audio::audio_manager::AudioManager, entities::util::master_entity_list::MasterEntityList, events::{event_handler::EventHandler, piano_sequences}, input::{key_states::KeyStates, piano::Piano}, scenes::scene_manager::SceneManager, util::master_clock::MasterClock};

pub struct EngineController {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    engine_controller: FrameworkController,
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
        let event_handler = EventHandler::new(self.master_entity_list.clone(), master_graphics_list.clone(), self.audio_manager.clone());

        self.set_resolution(1280.0, 720.0);

        // Go into this function to see how the loading is done.
        self.load_resources(&texture_manager.write().unwrap(), &master_graphics_list.write().unwrap());

        // Create a Piano instance
        let mut piano = Piano::new(self.audio_manager.clone(), self.key_states.clone());

        let mut flag = false;

        while flag == false {
            flag = self.main_loop(&event_handler, master_graphics_list.clone(), &mut piano);
        }
    }


    /// This is the main loop for the framework.
    /// It can contain game logic for now since we aren't abstracting much
    pub fn main_loop(&mut self, event_handler: &EventHandler, master_graphics_list: Arc<RwLock<MasterGraphicsList>>, piano: &mut Piano) -> bool {

        // Only uncomment this line if you want tons of information dumped into the console
        //master_graphics_list.read().unwrap().debug_all();

        self.execute_tick();

        // Thou shalt not use frame-based physics.
        let delta_time = self.master_clock.read().unwrap().get_delta_time();

        if self.window.should_close() {
            return true;
        }

        // Retrieve the "player" square from the master graphics list
        let square = master_graphics_list.read().unwrap().get_object("testscene_playersquare").expect("Object not found");

        // Just here for now, do movement inputs
        self.process_player_movement(square, delta_time);

        // Process piano inputs (returns true if a piano input was made)
        if piano.process_piano_keys() {
            piano_sequences::check_piano_sequences(piano, &event_handler);
        }

        // Spin this object for testing
        if let Some(object_2) = master_graphics_list.read().unwrap().get_object("testscene_obj1") {
            let mut object_2_read = object_2.write().unwrap();
            let rotfactor = object_2_read.get_rotation()+1.0*delta_time;
            object_2_read.set_rotation(rotfactor);
        } else {
            println!("No object found with name testscene_obj1.");
        }

        // Call the collision checking method
        let collision_events = event_handler.check_entity_collisions();
        event_handler.handle_collision_events(collision_events);

        return false;
    }

    /// Executing a tick renders the frame, updates the master clock, plays all queued audio, and updates inputs.
    pub fn execute_tick(&mut self) {
        // Update the clock
        self.master_clock.write().unwrap().update();

        let _ = self.audio_manager.write().unwrap().process_audio_queue();

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
        let mut scene_manager = self.scene_manager.write().unwrap();
        let audio_manager = self.audio_manager.read().unwrap();

        // Load the texture files and the scenes from their respective directories into memory
        let _ = texture_manager.load_textures_from_directory("src\\resources\\textures");
        let _ = texture_manager.load_textures_from_directory("src\\resources\\localonly\\textures");
        let _ = scene_manager.load_scenes_from_directory("src\\resources\\scenes", &texture_manager);
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\sounds\\piano");
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\sounds\\pianosequences");
        // Load resources which should not be uploaded
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\localonly\\music");
        let _ = audio_manager.load_sounds_from_directory("src\\resources\\localonly\\sounds");

        // Load the test scene from the manager into the master graphics list
        let scene_name = "testscene";
        scene_manager.load_scene_into_lists(&self.master_entity_list.read().unwrap(), master_graphics_list, scene_name.to_owned());
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

    pub fn set_resolution(&mut self, width: f32, height: f32) {
        self.window.set_size(width as i32, height as i32);
        self.engine_controller.set_resolution(width, height);
    }
}