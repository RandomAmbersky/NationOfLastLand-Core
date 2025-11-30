use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
