use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use shared::{decoder::ReceiveFromStream, encoder::SendToWriter, messages::client::AuthenticationResponse, types::Hwid, ADDR, PORT};
use std::{
    io::{self, Cursor, Seek},
    thread::sleep,
    time::Duration,
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

        let v = vec![];
        let mut cursor = Cursor::new(v);

        response.send(&mut cursor).await.unwrap();
        println!("Encoded {cursor:?}");

        let rec = AuthenticationResponse::receive(&mut cursor).await.unwrap();
        println!("{rec:?}");
    });

    // if let Ok(stream) = TcpStream::connect(format!("{ADDR}:{PORT}")) {
    //     println!("> Connected to the server!");
    // } else {
    //     panic!("> Couldn't connect to server...");
    // }

    std::thread::sleep(Duration::from_secs(12312312));

    Ok(())
}
