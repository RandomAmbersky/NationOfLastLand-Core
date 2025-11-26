use serde::{Serialize, Serializer};

#[derive(Clone, Copy)]
pub struct Guid(pub u128);

impl Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}
