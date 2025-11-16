use crate::defines::Point;
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Target {
    pub value: Point,
}
