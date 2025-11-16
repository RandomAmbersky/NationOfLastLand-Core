use crate::defines::Point;
use crate::ecs_utils::find_nearest_waste;
use crate::modules::components::{IsMoving, IsWaitingTarget, Pos, Target};
use crate::modules::entities::Vehicle;
use hecs::World;

/// System that processes vehicles waiting for targets, assigns nearest waste, and changes their state
pub fn process_waiting_vehicles(world: &mut World) {
    // First, collect all vehicle entities that are waiting for targets
    let mut waiting_vehicles = Vec::new();
    for (entity, (pos, _vehicle, _waiting)) in
        world.query::<(&Pos, &Vehicle, &IsWaitingTarget)>().iter()
    {
        waiting_vehicles.push((entity, *pos));
    }

    // Then find targets for each waiting vehicle (immutable operations)
    let mut vehicle_targets = Vec::new();
    for (entity, pos) in waiting_vehicles {
        let nearest_waste = find_nearest_waste(world, pos);
        vehicle_targets.push((entity, nearest_waste));
    }

    // Finally, assign targets and change states (mutable operations)
    for (entity, nearest_waste) in vehicle_targets {
        if let Some(pos) = nearest_waste {
            // Assign target
            let target = Target {
                value: Point { x: pos.x, y: pos.y },
            };
            world.insert_one(entity, target).unwrap();

            // Remove waiting state
            world.remove_one::<IsWaitingTarget>(entity).unwrap();

            // Add moving state
            world.insert_one(entity, IsMoving {}).unwrap();
        }
    }
}
