use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use shared::{ADDR, PORT};
use std::{io, net::TcpStream};

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

    if let Ok(stream) = TcpStream::connect(format!("{ADDR}:{PORT}")) {
        println!("> Connected to the server!");
    } else {
        panic!("> Couldn't connect to server...");
    }

    Ok(())
}
