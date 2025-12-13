use crate::descriptions::Descriptions;
use crate::modules::components::Floors;
use crate::world_utils::get_base_type;
use hecs::{Entity, World};

pub fn can_attach_floor_to_base(
    world: &World,
    descriptions: &Descriptions,
    base: Entity,
    floor_type: &str,
) -> Result<(), String> {
    // Get base type
    let base_type = get_base_type(world, base)?;

    // Check if base type exists in descriptions
    let base_desc = descriptions.bases.get(&base_type)
        .ok_or(format!("Base type '{}' not found in descriptions", base_type))?;

    // Check if floor type exists in descriptions
    if !descriptions.floors.contains_key(floor_type) {
        return Err(format!("Floor type '{}' not found in descriptions", floor_type));
    }

    // Check if base has Floors component
    let mut query = world.query_one::<&Floors>(base)
        .map_err(|_| "Base does not have Floors component".to_string())?;
    let floors = query.get().ok_or("Base does not have Floors component")?;

    // Check if floors count is less than max_floors
    if floors.0.len() >= base_desc.max_floors as usize {
        return Err(format!("Cannot attach floor: maximum floors ({}) reached for base type '{}'", base_desc.max_floors, base_type));
    }

    Ok(())
}
