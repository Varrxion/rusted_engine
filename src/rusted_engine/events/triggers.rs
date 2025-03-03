use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TriggerType {
    Collision,
    Destruction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trigger {
    pub trigger_type: TriggerType,  // Trigger type (collision, destruction, etc.)
    pub conditions: TriggerConditions,  // The conditions are specific to each trigger type
    pub outcome: Option<String>,  // Optional outcome (e.g., action to take)
    pub target: Option<String>,   // Optional target (e.g., object to affect)
}

// Different condition structures for each trigger type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TriggerConditions {
    CollisionConditions(CollisionCondition),
    DestructionConditions(DestructionCondition),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionCondition {
    pub object_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestructionCondition {
    pub object_name: String,
}