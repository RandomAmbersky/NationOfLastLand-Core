use crate::modules::components::Pos;
use crate::modules::entities::Waste;
use hecs::World;

pub fn find_nearest_waste(world: &World, from: Pos) -> Option<Pos> {
    let mut nearest: Option<Pos> = None;
    let mut min_distance_squared = f32::INFINITY;

    for (_entity, (pos, _waste)) in world.query::<(&Pos, &Waste)>().iter() {
        let dx = pos.x - from.x;
        let dy = pos.y - from.y;
        let distance_squared = dx * dx + dy * dy;

        if distance_squared < min_distance_squared {
            min_distance_squared = distance_squared;
            nearest = Some(*pos);
        }
    }

    nearest
}
