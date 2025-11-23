use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum UnitState {
    IsMoving,
    IsWaitingTarget,
    IsStopped
}