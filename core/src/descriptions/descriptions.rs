// Re-export для обратной совместимости или удобства
pub use super::*;
use std::{collections::{HashMap, HashSet}, error::Error};

/// Компонент для хранения базовых описаний различных юнитов, алертов и предметов
#[derive(Debug, Default)]
pub struct Descriptions {
    /// Описания юнитов, где ключ - название юнита, значение - описание
    pub units: UnitsDescriptions,
    /// Описания алертов, где ключ - тип алерта, значение - описание
    pub alerts: AlertsDescriptions,
    /// Предметы, где ключ - название предмета, значение - данные предмета
    pub items: HashMap<String, ItemYaml>,
    /// Транспортные средства, где ключ - название транспорта, значение - данные транспорта
    pub vehicles: HashMap<String, VehicleYaml>,
    /// Список типов повреждений
    pub damage_types: Vec<String>,
}
 
impl Descriptions {
    /// Валидирует соответствия attack_types.type из предметов с damage_types
    pub fn validate_attack_types(&self) -> Result<(), Box<dyn Error>> {
        let valid_damage_types: HashSet<&String> =
            self.damage_types.iter().collect();

        for (item_name, item) in &self.items {
            for (_attack_name, damages) in &item.attack_types {
                for attack in damages {
                    if !valid_damage_types.contains(&attack.attack_type) {
                        return Err(format!(
                            "Invalid attack type '{}' in item '{}'. Must match one of: {:?}",
                            attack.attack_type, item_name, self.damage_types
                        ).into());
                    }
                }
            }
        }

        Ok(())
    }
}
