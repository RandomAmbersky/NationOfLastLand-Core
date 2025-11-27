use std::collections::HashMap;
use crate::modules::components::Guid;
use hecs::Entity;

#[derive(Debug, Default)]
pub struct WorldState {
    pub entity_href_to_guid: HashMap<Guid, Entity>,
}
