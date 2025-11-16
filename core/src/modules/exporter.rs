use crate::modules::components::{MaxSpeed, Pos, Rot, ToxicPower};
use crate::modules::entities::Vehicle;
use crate::modules::entities::Waste;
use crate::modules::state::State;
use hecs::World;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
struct ExportData {
    wastes: Vec<HashMap<String, Value>>,
    vehicles: Vec<HashMap<String, Value>>,
    state: State,
}

pub fn export_to_json(world: &World, state: &State) -> String {
    let mut wastes = Vec::new();
    let mut vehicles = Vec::new();

    // Выборка всех waste
    for (_id, (pos, toxic_power, _waste)) in world.query::<(&Pos, &ToxicPower, &Waste)>().iter() {
        wastes.push(HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
            ("toxic_power".to_string(), serde_json::to_value(*toxic_power).unwrap()),
        ]));
    }

    // Выборка всех vehicle
    for (_id, (pos, rot, max_speed, _vehicle)) in world.query::<(&Pos, &Rot, &MaxSpeed, &Vehicle)>().iter() {
        vehicles.push(HashMap::from([
            ("id".to_string(), Value::Number(_id.id().into())),
            ("pos".to_string(), serde_json::to_value(*pos).unwrap()),
            ("rot".to_string(), serde_json::to_value(*rot).unwrap()),
            ("max_speed".to_string(), serde_json::to_value(*max_speed).unwrap()),
        ]));
    }

    let data = ExportData {
        wastes,
        vehicles,
        state: state.clone(),
    };
    serde_json::to_string(&data).unwrap()
}
