use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Floors(pub u32);
