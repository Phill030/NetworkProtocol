use std::{io, net::TcpStream};

use shared::{ADDR, PORT};

fn main() -> io::Result<()> {
    if let Ok(stream) = TcpStream::connect(format!("{ADDR}:{PORT}")) {
        println!("> Connected to the server!");
    } else {
        panic!("> Couldn't connect to server...");
    }

    Ok(())
}
