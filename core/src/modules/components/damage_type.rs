use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum DamageType {
    Physical,   // можно разбить на Slash, Pierce и т.д.
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Magic,
}