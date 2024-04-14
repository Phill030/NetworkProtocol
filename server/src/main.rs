use crate::client::Client;
use shared::{ADDR, PORT};
use std::{
    io::{self, Read},
    net::TcpListener,
};
use tokio::runtime::Runtime;

mod client;

fn main() -> io::Result<()> {
    let rt = Runtime::new()?;
    let listener = TcpListener::bind(format!("{ADDR}:{PORT}"))?;

    loop {
        let (stream, addr) = listener.accept()?;

        println!("> {} connected", stream.peer_addr()?);
        rt.spawn(async move { handle_client(Client::new(stream, addr)).await });
    }
}

async fn handle_client(mut client: Client) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = client.stream.read(&mut buffer)?;
        if bytes_read == 0 {
            println!("> {} disconnected", client.addr);
            break;
        }

        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }

    Ok(())
}
