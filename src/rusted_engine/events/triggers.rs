use serde::{Deserialize, Serialize};

use crate::rusted_engine::{audio::audio_manager::AudioType, scenes::scene_manager::ObjectData};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trigger {
    pub trigger_type: TriggerType,  // Trigger type (collision, destruction, etc.)
    pub conditions: Option<TriggerConditions>,  // The conditions are specific to each trigger type
    pub outcome: Option<Outcome>,   // Outcome tells not only which action to take but also the arguments for the action
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneTrigger {
    pub scene_trigger_type: SceneTriggerType,
    pub conditions: Option<TriggerConditions>,
    pub outcome: Outcome,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SceneTriggerType {
    KeyPressed,
    Timer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TriggerType {
    Collision,
    Destruction,
}

// Different condition structures for each trigger type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TriggerConditions {
    CollisionConditions(CollisionCondition),
    KeyPressedConditions(KeyPressedCondition),
    TimerConditions(TimerCondition),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyPressedCondition {
    pub key_pressed: char,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimerCondition {
    pub time_in_seconds: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionCondition {
    pub collided_with: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Outcome {
    Sequence(SequenceArgs),
    SwapScene(SwapSceneArgs),
    CreateObject(ObjectData),
    DestroyObject(DestroyObjectArgs),
    TeleportObject(TeleportObjectArgs),
    EnqueueAudio(EnqueueAudioArgs),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  SequenceArgs {
    pub sequence_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  SwapSceneArgs {
    pub scene_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  DestroyObjectArgs {
    pub object_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  TeleportObjectArgs {
    pub object_name: String,
    pub new_position: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  EnqueueAudioArgs {
    pub audio_name: String,
    pub audio_type: AudioType,
    pub volume: f32,
    pub looping: bool,
}