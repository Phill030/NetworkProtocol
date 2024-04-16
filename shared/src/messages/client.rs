use crate::{decoder::Decoder, encoder::Encoder, message_type, messages::EncodeError, types::Hwid};
use macros::Networked;

#[derive(Debug)]
pub enum ClientPackets {
    AuthenticationResponse(AuthenticationResponse),
    KeepAliveResponse(KeepAliveResponse),
}

message_type!(ClientMessageType; AuthenticationResponse, KeepAliveResponse);

#[derive(Networked, Clone, Debug)]
#[packet_id(0x00)]
pub struct AuthenticationResponse {
    pub hwid: Hwid,
    pub nonce: String,
}

#[derive(Networked, Clone, Debug)]
#[packet_id(0x01)]
pub struct KeepAliveResponse {
    timestamp: i64,
}
