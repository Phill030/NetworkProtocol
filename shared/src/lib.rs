pub mod decoder;
pub mod encoder;
pub mod errors;
pub mod messages;
pub mod types;
pub mod utils;

pub const ADDR: &str = "127.0.0.1";
pub const PORT: u16 = 7776;

// Authentication flow:
//
//  AuthenticationRequest(Nonce): Server
//  AuthenticationResponse(Hwid, Nonce): Client
