use serde::Serialize;

#[derive(Serialize, Clone, Copy, Default)]
pub struct ToxicPower {
    pub level: f32,
}
