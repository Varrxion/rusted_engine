use std::{collections::{HashMap, HashSet}, fs::{self, File}, ops::Range, path::Path, sync::{Arc, RwLock}};

use nalgebra::{Vector2, Vector3};
use rusted_open::framework::graphics::{internal_object::{custom_shader::CustomShader, graphics_object::Generic2DGraphicsObject}, texture_manager::TextureManager, util::master_graphics_list::MasterGraphicsList};
use serde::Deserialize;
use std::io::{self, Read};

use crate::rusted_engine::{entities::{generic_entity::{CollisionMode, GenericEntity}, util::master_entity_list::MasterEntityList}, events::triggers::{CollisionCondition, DestructionCondition, Trigger, TriggerConditions}, game_state::GameState};

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

    pub fn load_scene(&self, game_state: &mut GameState, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, scene_name: String) {
        self.load_scene_into_game_state(game_state, scene_name.clone());
        self.load_scene_into_lists(master_entity_list, master_graphics_list, scene_name);
    }

    fn load_scene_into_game_state(&self, game_state: &mut GameState, scene_name: String) {
        if let Some(scene) = self.get_scene(&scene_name) {
            if let Ok(scene) = scene.read() {
                game_state.set_gravity(scene.get_gravity());
                game_state.set_terminal_velocity(scene.get_terminal_velocity());
            }
            else {
                println!("Could not get a read lock on Scene with name: {}", scene_name);
            }
        }
        else {
            println!("Could not find a Scene with name: {}", scene_name);
        }
    }

    fn load_scene_into_lists(&self, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, scene_name: String) {
        self.load_scene_into_master_entity_list(master_entity_list, scene_name.clone());
        self.load_scene_into_master_graphics_list(master_graphics_list, scene_name.clone());
    }

    fn load_scene_into_master_graphics_list(&self, master_graphics_list: &MasterGraphicsList, scene_name: String) {
        if let Some(scene) = self.get_scene(&scene_name) {
            if let Ok(scene) = scene.read() {
                for obj in scene.get_graphics_objects().iter() {
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

    fn load_scene_into_master_entity_list(&self, master_entity_list: &MasterEntityList, scene_name: String) {
        if let Some(scene) = self.get_scene(&scene_name) {
            if let Ok(scene) = scene.read() {
                for entity in scene.get_entities().iter() {
                    let cloned_entity = entity.read().unwrap().clone(); // Clone the actual entity
                    let arc_entity = Arc::new(RwLock::new(cloned_entity));
                    master_entity_list.add_entity(arc_entity);
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
        .map_err(|e| {
            eprintln!("Error parsing JSON: {}", e);
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;
    
        let gravity = match scene_data.properties.gravity.len() {
            2 => Vector2::new(scene_data.properties.gravity[0], scene_data.properties.gravity[1]),
            1 => Vector2::new(0.0, scene_data.properties.gravity[0]), // Interpret as gravity on the y-axis
            _ => Vector2::new(0.0, 0.0), // Default to (0.0, 0.0) if invalid
        };

        let terminal_velocity = match scene_data.properties.terminal_velocity.len() {
            2 => Vector2::new(scene_data.properties.terminal_velocity[0], scene_data.properties.terminal_velocity[1]),
            1 => Vector2::new(0.0, scene_data.properties.terminal_velocity[0]), // Interpret as terminal velocity on the y-axis
            _ => Vector2::new(f32::MAX, f32::MAX), // Default to (0.0, 0.0) if invalid
        };

        let mut json_scene = Scene::new(gravity, terminal_velocity);
    
        for obj_data in scene_data.objects {
            let json_shader = CustomShader::new(
                &obj_data.graphics.vertex_shader,
                &obj_data.graphics.fragment_shader,
            );
    
            let mut json_collision_modes = HashSet::new();
            for collision_mode in obj_data.entity.collision_modes {
                match collision_mode.as_str() {
                    "AABB" => { json_collision_modes.insert(CollisionMode::AABB); }
                    "Circle" => { json_collision_modes.insert(CollisionMode::Circle); }
                    "OBB" => { json_collision_modes.insert(CollisionMode::OBB); }
                    _ => {}
                }
            }
    
            let position = Vector3::new(
                obj_data.graphics.position[0],
                obj_data.graphics.position[1],
                obj_data.graphics.position[2],
            );
    
            let texture_id = texture_manager.get_texture_id(&obj_data.graphics.texture_name);
    
            let graphics_object = Generic2DGraphicsObject::new(
                obj_data.graphics.name.clone(),
                obj_data.graphics.vertex_data,
                obj_data.graphics.texture_coords,
                json_shader.get_shader_program(),
                position,
                obj_data.graphics.rotation,
                obj_data.graphics.scale,
                texture_id,
                obj_data.graphics.uses_atlas,
                obj_data.graphics.current_frame,
                obj_data.graphics.frame_range,
                obj_data.graphics.frame_duration,
                obj_data.graphics.atlas_columns,
                obj_data.graphics.atlas_rows,
            );

            let velocity = obj_data.entity.velocity.unwrap_or_else(|| vec![0.0, 0.0]);
            let velocity_vector = Vector2::new(velocity[0], velocity[1]);

            // Default collision_priority to 0 if None
            let collision_priority = obj_data.entity.collision_priority.unwrap_or(0);

            let triggers: Vec<Trigger> = obj_data.entity.triggers.unwrap_or_default().into_iter().map(|trigger_data| {
                let conditions = match trigger_data.conditions {
                    TriggerConditions::CollisionConditions(cond) => TriggerConditions::CollisionConditions(CollisionCondition {
                        object_name: cond.object_name,
                    }),
                    TriggerConditions::DestructionConditions(cond) => TriggerConditions::DestructionConditions(DestructionCondition {
                        object_name: cond.object_name,
                    }),
                };
            
                Trigger {
                    trigger_type: trigger_data.trigger_type,
                    conditions,
                    outcome: trigger_data.outcome,
                    target: trigger_data.target,
                }
            }).collect();
            
    
            let entity = GenericEntity::new(
                obj_data.entity.name.clone(),
                obj_data.entity.weight,
                velocity_vector,
                obj_data.entity.affected_by_gravity,
                obj_data.entity.is_static,
                obj_data.entity.elasticity,
                obj_data.entity.active_collision,
                collision_priority,
                json_collision_modes,
                triggers,
            );
    
            let wrapped_graphics_object = Arc::new(RwLock::new(graphics_object));
            let wrapped_entity = Arc::new(RwLock::new(entity));
    
            json_scene.add_graphics_object(wrapped_graphics_object);
            json_scene.add_entity(wrapped_entity);
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
            println!("The file name is: {}", file_name);
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
struct EntityData {
    name: String,
    weight: f32,
    velocity: Option<Vec<f32>>,
    affected_by_gravity: bool,
    is_static: bool,
    elasticity: f32,
    active_collision: bool,
    #[serde(default)]
    collision_priority: Option<u64>,
    collision_modes: Vec<String>,
    triggers: Option<Vec<Trigger>>,
}

#[derive(Deserialize)]
struct GraphicsData {
    name: String,
    vertex_data: Vec<f32>,
    texture_coords: Vec<f32>,
    vertex_shader: String,
    fragment_shader: String,
    position: Vec<f32>,
    rotation: f32,
    scale: f32,
    texture_name: String,
    uses_atlas: bool,
    current_frame: usize,
    frame_range: Range<usize>,
    frame_duration: f32,
    atlas_columns: usize,
    atlas_rows: usize,
}

#[derive(Deserialize)]
struct ObjectData {
    entity: EntityData,
    graphics: GraphicsData,
}

#[derive(Deserialize)]
struct SceneData {
    properties: SceneProperties,
    objects: Vec<ObjectData>,
}

#[derive(Deserialize)]
struct SceneProperties {
    gravity: Vec<f32>,
    terminal_velocity: Vec<f32>,
}