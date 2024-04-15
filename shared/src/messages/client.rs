use crate::decoder::Decoder;
use crate::encoder::Encoder;
use crate::types::Hwid;
use macros::{Receivable, Serialize};

#[derive(Serialize, Clone, Receivable)]

pub struct AuthenticationResponse {
    pub hwid: Hwid,
    pub nonce: String,
}
