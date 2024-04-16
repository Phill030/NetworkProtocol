use crate::{decoder::Decoder, encoder::Encoder, message_type, messages::EncodeError};
use macros::Networked;
use std::time::{SystemTime, UNIX_EPOCH};
use textnonce::TextNonce;

#[derive(Debug)]
pub enum ServerPackets {
    AuthenticationRequest(AuthenticationRequest),
    KeepAliveRequest(KeepAliveRequest),
}

message_type!(ServerMessageType; AuthenticationRequest, KeepAliveRequest);

#[derive(Networked, Clone, Debug)]
#[packet_id(0x00)]
pub struct AuthenticationRequest {
    pub nonce: String,
}
impl AuthenticationRequest {
    pub fn new() -> Self {
        Self {
            nonce: TextNonce::sized(16).unwrap().into_string(),
        }
    }
}

#[derive(Networked, Clone, Debug)]
#[packet_id(0x01)]
pub struct KeepAliveRequest {
    pub timestamp: i64,
}
impl KeepAliveRequest {
    pub fn new() -> Self {
        Self {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        }
    }
}
