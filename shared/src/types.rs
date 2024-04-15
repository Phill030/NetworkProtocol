use crate::encoder::Encoder;
use macros::Serialize;

#[derive(Serialize, Clone)]
pub struct Hwid {
    pub cpu_id: String,
    pub system_id: String,
}
