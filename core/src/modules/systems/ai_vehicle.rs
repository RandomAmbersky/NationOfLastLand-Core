use crate::modules::components::Pos;
use crate::modules::entities::Vehicle;
use crate::ecs_utils::find_nearest_waste;
use hecs::World;

/// System that finds all vehicles and determines their nearest waste targets
pub fn ai_vehicle_system(world: &World) -> Vec<(hecs::Entity, Option<Pos>)> {
    let mut vehicle_targets = Vec::new();

    // Query for all entities with Pos and Vehicle components
    for (entity, (pos, _vehicle)) in world.query::<(&Pos, &Vehicle)>().iter() {
        // Find the nearest waste for this vehicle
        let nearest_waste = find_nearest_waste(world, *pos);
        vehicle_targets.push((entity, nearest_waste));
    }

    vehicle_targets
}
