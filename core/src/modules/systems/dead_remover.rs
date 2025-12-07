use hecs::World;
use crate::modules::markers::IsDead;

pub fn do_remove_dead(world: &mut World) {
    // Collect entities with IsDead marker
    let mut entities_to_remove: Vec<hecs::Entity> = Vec::new();

    for (entity, _is_dead) in world.query::<&IsDead>().iter() {
        entities_to_remove.push(entity);
    }

    // Remove the collected entities
    for entity in entities_to_remove {
        world.despawn(entity).unwrap();
    }
}
