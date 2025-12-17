// SHARED

pub const TICK_RATE: f32 = 64.0;

pub const ROLLBACK_WINDOW: u64 = 120;
pub fn serialize<T: serde::Serialize>(value: &T) -> Vec<u8> {
    bincode::serialize(value).unwrap()
}

pub fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T {
    bincode::deserialize(bytes).unwrap()
}
