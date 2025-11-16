use serde::Serialize;

// юнит двигается к цели
#[derive(Serialize, Clone)]
pub struct IsMoving {}

// юнит стоит рядом с целью
#[derive(Serialize, Clone)]
pub struct IsStopped {}

// юнит в поиске цели
#[derive(Serialize, Clone)]
pub struct IsWaitingTarget {}
