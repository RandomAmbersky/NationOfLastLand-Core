use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Copy, Serialize)]
pub struct Guid(pub Uuid);

impl Guid {
    pub fn new() -> Self {
        Guid(Uuid::new_v4())
    }
}
