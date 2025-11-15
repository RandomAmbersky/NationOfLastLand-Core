use serde::Serialize;

#[derive(Serialize, Clone, Copy, Default)]
pub struct Reputation {
    pub value: f32,
}
