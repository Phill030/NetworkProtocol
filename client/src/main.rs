use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use shared::{encoder::Encoder, messages::client::AuthenticationResponse, types::Hwid, ADDR, PORT};
use std::{
    io::{self, Cursor},
    net::TcpStream,
};
use tokio::runtime::Runtime;

const KEY: &str = "HASHING_KEY";

fn main() -> io::Result<()> {
    let cpu_id = IdBuilder::new(Encryption::SHA256)
        .add_component(HWIDComponent::CPUID)
        .build(KEY)
        .unwrap();
    let system_id = IdBuilder::new(Encryption::SHA256)
        .add_component(HWIDComponent::SystemID)
        .build(KEY)
        .unwrap();

    Runtime::new().unwrap().spawn(async move {
        let response = AuthenticationResponse {
            hwid: Hwid { cpu_id, system_id },
            nonce: "ABC".to_string(),
        };

        let v = Vec::with_capacity(1024);
        let mut cursor = Cursor::new(v);

        response.encode(&mut cursor).await.unwrap();
        println!("Encoded {cursor:?}");
    });

    if let Ok(stream) = TcpStream::connect(format!("{ADDR}:{PORT}")) {
        println!("> Connected to the server!");
    } else {
        panic!("> Couldn't connect to server...");
    }

    Ok(())
}
