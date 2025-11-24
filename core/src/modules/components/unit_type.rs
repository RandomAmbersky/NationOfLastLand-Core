use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum UnitType {
    Vehicle,
    Waste,
    Trash
}
