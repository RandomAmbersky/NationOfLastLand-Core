use nation_of_last_land_core::Core;

fn main() {
    // Создание Core и автоматический вызов load() в конструкторе
    let core = Core::new();

    // Получение описаний, загруженных из YAML
    let descriptions = core.get_descriptions();

    // Вывод типов повреждений
    println!("Damage Types:");
    for (index, damage_type) in descriptions.damage_types.iter().enumerate() {
        println!("  {}. {}", index + 1, damage_type);
    }

    // Вывод предметов
    println!("\nItems:");
    for (name, item) in &descriptions.items {
        println!("  {}:", name);
        for attack_type in &item.attack_types {
            println!("    - {}: damage={}", attack_type.attack_type, attack_type.damage);
        }
    }
}
