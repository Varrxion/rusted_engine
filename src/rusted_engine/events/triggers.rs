use serde::{Deserialize, Serialize};

use crate::rusted_engine::audio::audio_manager::AudioType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trigger {
    pub trigger_type: TriggerType,  // Trigger type (collision, destruction, etc.)
    pub conditions: Option<TriggerConditions>,  // The conditions are specific to each trigger type
    pub outcome: Option<Outcome>,   // Outcome tells not only which action to take but also the arguments for the action
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionCondition {
    pub collided_with: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Outcome {
    Sequence(SequenceArgs),
    SwapScene(SwapSceneArgs),
    DestroyObject(DestroyObjectArgs),
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
pub struct  EnqueueAudioArgs {
    pub audio_name: String,
    pub audio_type: AudioType,
    pub volume: f32,
    pub looping: bool,
}