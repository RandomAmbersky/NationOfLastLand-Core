use crate::defines::MinMax;
use crate::descriptions::{Descriptions, load_damage_types_static, load_items_static, load_vehicles_static};
use crate::modules::components::{AttachedItems, Owner, Pos};
use crate::modules::markers::Item;
use crate::modules::systems::dead_remover::do_remove_dead;
use crate::modules::systems::move_system::do_move;
use crate::world_utils::{get_base_type, spawn_entity};

use crate::modules::exporter::{export_to_json, export_entity_to_json};
use crate::modules::setup;
use crate::modules::state::State;
use crate::modules::systems::ai_vehicle::{ai_vehicle_system};
use crate::modules::systems::attack_processor::attack_process;
use crate::random_generator::RandomGenerator;
use crate::spawner::{create_item_from_yaml, create_vehicle_from_yaml};
use hecs::{Entity, World};
use std::error::Error;

const DAMAGE_TYPES_YAML: &str = include_str!("../../data/damage_types.yml");
const ITEMS_YAML: &str = include_str!("../../data/items.yml");
const VEHICLES_YAML: &str = include_str!("../../data/vehicles.yml");

pub struct Core {
    world: World,
    r: RandomGenerator,
    s: State,
    setup: setup::Setup,
    descriptions: Descriptions,
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Core {
    pub fn new() -> Self {
        let world = World::new();
        let s = State::new();
        let setup = setup::new();
        let r = RandomGenerator {
            toxic_health: MinMax { max: 5.0, min: 1.0 },
        };
        let descriptions = Descriptions::default();

        let mut c = Core {
            world,
            s,
            r,
            setup,
            descriptions,
        };

        c.load().expect("Failed to load damage types");

        c.init_world();
        c
    }

    pub fn create_trash(&mut self) -> Result<(), String> {
        let bundle = self.r.get_bundle_trash(&self.setup.spatial.map_size);
        spawn_entity(&mut self.world, bundle);
        Ok(())
    }

    pub fn create_vehicle(&mut self, vehicle_key: &str, pos: Pos) -> Result<Entity, String> {
        create_vehicle_from_yaml(&mut self.world, &self.descriptions, vehicle_key, pos)
    }

    pub fn create_item(&mut self, item_key: &str, pos: Pos) -> Result<Entity, String> {
        create_item_from_yaml(&mut self.world, &self.descriptions, item_key, pos)
    }

    pub fn update(&mut self, delta: f64) -> Result<(), String> {

        do_remove_dead(&mut self.world);

        do_move(&mut self.world, &self.setup.spatial);

        // Run AI system to process waiting vehicles and assign targets
        ai_vehicle_system(&mut self.world, &self.descriptions);

        // Process attack events and apply damage
        attack_process(&mut self.world);

        // Increment time
        self.s.time += delta;

        Ok(())
    }

    pub fn get_descriptions(&self) -> &Descriptions {
        &self.descriptions
    }

    pub fn get_descriptions_mut(&mut self) -> &mut Descriptions {
        &mut self.descriptions
    }

    pub fn attach_to_vehicle(&mut self, vehicle: Entity, item: Entity, slot_id: &str) -> Result<(), String> {
        // Get vehicle type
        let vehicle_type = get_base_type(&self.world, vehicle)?;

        // Check if slot exists in descriptions
        let has_slot = if let Some(vehicle_desc) = self.descriptions.vehicles.get(&vehicle_type) {
            vehicle_desc.active_slot.iter().any(|slot| slot.id == slot_id)
        } else {
            false
        };

        if has_slot {
            // Check if item is an item
            if self.world.get::<&Item>(item).is_ok() {
                // Insert Owner component to item
                self.world.insert_one(item, Owner(vehicle)).expect("Failed to insert Owner component");

                if self.world.get::<&AttachedItems>(vehicle).is_err() {
                    self.world.insert_one(vehicle, AttachedItems::new()).unwrap();
                }

                let mut query = self.world.query_one::<&mut AttachedItems>(vehicle).unwrap();
                let attached_items = query.get().unwrap();
                attached_items.attach(slot_id, item);

                Ok(())
            } else {
                Err("Entity is not an item".to_string())
            }
        } else {
            Err(format!("Slot '{}' not found on vehicle '{}'", slot_id, vehicle_type))
        }
    }

    pub fn export_world(&self, is_pretty: bool) -> String {
        export_to_json(&self.world, &self.s, is_pretty)
    }

    // Export once entity
    pub fn export_entity(&self, entity: Entity, is_pretty: bool) -> String {
        export_entity_to_json(&self.world, entity, is_pretty)
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.descriptions.damage_types = load_damage_types_static(DAMAGE_TYPES_YAML)?;
        self.descriptions.items = load_items_static(ITEMS_YAML)?.items;
        self.descriptions.vehicles = load_vehicles_static(VEHICLES_YAML)?.vehicles;
        self.descriptions.validate_attack_types()?;
        Ok(())
    }
}

impl Core {
    fn init_world(&mut self) {
        // Создание vehicle на основе данных из YAML (VEHICLE_CAR)
        let vehicle = self.create_vehicle("VEHICLE_CAR", Pos { x: 1.0, y: 1.0 })
            .expect("Failed to create vehicle from YAML");

        // Create an item with interactions
         let item = self.create_item("ITEM_CLEANER", Pos { x: 0.0, y: 0.0 }).unwrap();

        // Attach item to vehicle
        self.attach_to_vehicle(vehicle, item, "front_left").unwrap();

        for _ in 0..2 {
            self.create_trash().expect("Failed to create waste");
        }
    }
}
