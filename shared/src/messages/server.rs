use crate::encoder::Encoder;
use macros::Serialize;

#[derive(Serialize, Clone)]
pub struct AuthenticationRequest {
    pub nonce: String,
}
