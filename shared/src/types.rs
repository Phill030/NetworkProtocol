use crate::{encoder::Encoder, errors::decode::DecodeError};
use macros::{Deserialize, Serialize};
use tokio::io::AsyncRead;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hwid {
    pub cpu_id: String,
    pub system_id: String,
}
