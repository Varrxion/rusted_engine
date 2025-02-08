use std::{collections::{HashMap, HashSet}, fs::{self, File}, path::Path, sync::{Arc, RwLock}};

use nalgebra::Vector3;
use rusted_open::framework::graphics::{internal_object::{custom_shader::CustomShader, graphics_object::{CollisionMode, Generic2DGraphicsObject}}, texture_manager::TextureManager, util::master_graphics_list::{self, MasterGraphicsList}};
use serde::Deserialize;
use std::io::{self, Read};

use super::scene::Scene;

pub struct SceneManager {
    scenes: HashMap<String, Arc<RwLock<Scene>>>, // Use RwLock for thread safety
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
        }
    }

    /// Adds a new scene to the manager.
    pub fn add_scene(&mut self, name: String, scene: Scene) {
        self.scenes.insert(name, Arc::new(RwLock::new(scene)));
    }

    /// Retrieves a scene by its name.
    pub fn get_scene(&self, name: &str) -> Option<Arc<RwLock<Scene>>> {
        self.scenes.get(name).cloned()
    }

    /// Removes a scene by its name.
    pub fn remove_scene(&mut self, name: &str) -> Option<Arc<RwLock<Scene>>> {
        self.scenes.remove(name)
    }

    /// Lists all scene names.
    pub fn list_scenes(&self) -> Vec<String> {
        self.scenes.keys().cloned().collect()
    }

    pub fn load_scene_into_master_graphics_list(&self, master_graphics_list: &MasterGraphicsList, scene_name: String) {
        if let Some(scene) = self.get_scene(&scene_name) {
            if let Ok(scene) = scene.read() {
                for obj in scene.get_objects().iter() {
                    let cloned_obj = obj.read().unwrap().clone(); // Clone the actual object
                    let arc_obj = Arc::new(RwLock::new(cloned_obj));
                    master_graphics_list.add_object(arc_obj);
                }
            }
            else {
                println!("Could not get a read lock on Scene with name: {}", scene_name);
            }
        }
        else {
            println!("Could not find a Scene with name: {}", scene_name);
        }
    }

    pub fn load_scene_from_json(&mut self, file_path: &str, texture_manager: &TextureManager) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
    
        let scene_data: SceneData = serde_json::from_str(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
        let mut json_scene = Scene::new();
    
        for obj_data in scene_data.objects {
            let json_shader = CustomShader::new(
                &obj_data.vertex_shader,
                &obj_data.fragment_shader,
            );
    
            let mut json_collision_modes = HashSet::new();
            for collision_mode in obj_data.collision_modes {
                match collision_mode.as_str() {
                    "AABB" => { json_collision_modes.insert(CollisionMode::AABB); }
                    "Circle" => { json_collision_modes.insert(CollisionMode::Circle); }
                    "OBB" => { json_collision_modes.insert(CollisionMode::OBB); }
                    _ => {}
                }
            }
    
            let position = Vector3::new(
                obj_data.position[0],
                obj_data.position[1],
                obj_data.position[2],
            );
    
            let texture_id = texture_manager.get_texture_id(&obj_data.texture_name);
    
            let graphics_object = Generic2DGraphicsObject::new(
                obj_data.name,
                obj_data.vertex_data,
                obj_data.texture_coords,
                json_shader.get_shader_program(),
                position,
                obj_data.rotation,
                obj_data.scale,
                texture_id,
                json_collision_modes,
            );
    
            let wrapped_object = Arc::new(RwLock::new(graphics_object));
    
            json_scene.add_object(wrapped_object);
        }
    
        let scene_name = Path::new(file_path)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("Unnamed")
            .to_string();
    
        self.add_scene(scene_name, json_scene);
    
        Ok(())
    }

    /// Loads all scenes from JSON files in the specified directory
    pub fn load_scenes_from_directory(&mut self, dir_path: &str, texture_manager: &TextureManager) -> Result<(), String> {
        let paths = fs::read_dir(dir_path).map_err(|_| "Failed to read directory".to_string())?;

        for path in paths {
            let entry = path.map_err(|_| "Failed to read directory entry".to_string())?;
            let file_name = entry.file_name().into_string().map_err(|_| "Invalid file name".to_string())?;
            let full_path = entry.path();

            // Only load JSON files
            if full_path.is_file() {
                if let Some(extension) = full_path.extension() {
                    if extension == "json" {
                        // Load the scene with the file name
                        self.load_scene_from_json(full_path.to_str().unwrap(), texture_manager)
                            .map_err(|e| format!("Error loading scene '{}': {}", file_name, e))?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct ObjectData {
    name: String,
    vertex_data: Vec<f32>,
    texture_coords: Vec<f32>,
    vertex_shader: String,
    fragment_shader: String,
    position: Vec<f32>,  // [x, y, z]
    rotation: f32,
    scale: f32,
    texture_name: String,
    collision_modes: Vec<String>,
}

#[derive(Deserialize)]
struct SceneData {
    objects: Vec<ObjectData>,
}