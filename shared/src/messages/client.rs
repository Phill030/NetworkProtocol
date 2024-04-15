use crate::{decoder::Decoder, encoder::Encoder, types::Hwid};
use macros::{Receivable, Streamable};
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[derive(Streamable, Clone, Debug, Receivable)]
#[packet_id(0x66)]
pub struct AuthenticationResponse {
    pub hwid: Hwid,
    pub nonce: String,
}
