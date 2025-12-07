use crate::descriptions::Descriptions;
use crate::modules::components::{BaseType, EntityType, Force, Health, MaxSpeed, Pos, Rot, Velocity, WeaponMode, WeaponType};
use crate::modules::markers::{IsWaitingTarget, Vehicle, Item};
use crate::random_generator::RandomGenerator;
use crate::world_utils::spawn_entity;
use hecs::{Entity, World};

pub fn create_vehicle_from_description(world: &mut World, descriptions: &Descriptions, vehicle_key: &str, pos: Pos) -> Result<Entity, String> {
    if let Some(vehicle_data) = descriptions.vehicles.get(vehicle_key) {
        let e = spawn_entity(world, (
            BaseType(vehicle_key.to_string()),
            pos,
            Rot { x: 0.0, y: 0.0 },
            MaxSpeed(vehicle_data.max_speed),
            Velocity { x: 0.0, y: 0.0 },
            Health { current: vehicle_data.max_health, max: vehicle_data.max_health },
            Force(100.0),
            IsWaitingTarget {},
            EntityType::Vehicle,
            Vehicle {},
        ));

        Ok(e)
    } else {
        Err(format!("Vehicle '{}' not found in descriptions", vehicle_key))
    }
}

pub fn create_item_from_description(world: &mut World, descriptions: &Descriptions, item_key: &str, _pos: Pos) -> Result<Entity, String> {
    if let Some(item_data) = descriptions.items.get(item_key) {
        let mut modes = Vec::new();
        for interaction in &item_data.interactions {
            for (dmg_type, dmg_value) in &interaction.action {
                modes.push(WeaponMode {
                    damage_type: dmg_type.clone(),
                    damage: *dmg_value,
                    range: 1.0,
                });
            }
        }
        let e = spawn_entity(world, (
            BaseType(item_key.to_string()),
            EntityType::Item,
            Item {},
        ));
        if !modes.is_empty() {
            let weapon_type = WeaponType { modes };
            world.insert_one(e, weapon_type).unwrap();
        }
        Ok(e)
    } else {
        Err(format!("Item '{}' not found in descriptions", item_key))
    }
}

pub fn create_alert_from_description(world: &mut World, descriptions: &Descriptions, alert_key: &str, pos: Pos, r: &RandomGenerator) -> Result<Entity, String> {
    if let Some(_description) = descriptions.alerts.get(alert_key) {
        match alert_key {
            "ALERT_TRASH" => Ok(spawn_entity(world, r.get_bundle_trash(pos))),
            "ALERT_WASTE" => Ok(spawn_entity(world, r.get_bundle_waste(pos))),
            _ => Err(format!("Unknown alert type '{}'", alert_key)),
        }
    } else {
        Err(format!("Alert '{}' not found in descriptions", alert_key))
    }
}
