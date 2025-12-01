use hecs::Entity;
use crate::modules::components::WeaponType;

#[derive(Clone, Debug)]
pub struct Weapon {
    pub owner: Option<Entity>,
    pub weapon_type: WeaponType
}
