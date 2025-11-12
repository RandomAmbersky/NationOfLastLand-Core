use crate::modules::components::pos::Pos;

mod modules;

fn main() {
    let core = modules::core::Core::new();

    core.get_manager_create()
        .create_waste(Pos { x: 10.0, y: 10.0 })
        .unwrap();

    core.get_manager_create()
        .create_vehicle(Pos { x: 20.0, y: 20.0 })
        .unwrap();

    println!("Hello, world!");
}
