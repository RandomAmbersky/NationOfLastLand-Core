use serde::{Serialize, Deserialize};

use crate::modules::components::Guid;

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct TargetId(pub Guid);
