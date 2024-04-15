use crate::types::Hwid;

pub struct AuthenticationResponse {
    pub hwid: Hwid,
    pub nonce: String,
}
