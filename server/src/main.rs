use shared::{
    decoder::ReceiveFromStream,
    messages::{
        client::{AuthenticationResponse, ClientMessageType},
        server::{AuthenticationRequest, KeepAliveRequest, ServerPackets},
        SystemPacket,
    },
    ADDR, PORT,
};
use std::{
    io::{self, Cursor},
    net::SocketAddr,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::OwnedReadHalf, TcpListener},
    spawn,
    sync::mpsc::{channel, Sender},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{ADDR}:{PORT}")).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let (reader, mut writer) = stream.into_split();
        let (sender, mut receiver) = channel::<ServerPackets>(100);
        let keep_alive_sender = sender.clone();

        spawn(async move { handle_client(addr, reader, sender).await });
        spawn(async move { keep_alive(keep_alive_sender).await });

        while let Some(recv) = receiver.recv().await {
            let buffer = match recv {
                ServerPackets::AuthenticationRequest(x) => x.to_bytes().await,
                ServerPackets::KeepAliveRequest(x) => x.to_bytes().await,
                _ => panic!("Invalid packet received!"),
            }
            .unwrap();

            writer.write_all(&buffer[..]).await.unwrap();
        }
    }
}

async fn handle_client(addr: SocketAddr, mut reader: OwnedReadHalf, sender: Sender<ServerPackets>) -> io::Result<()> {
    sender
        .send(ServerPackets::AuthenticationRequest(AuthenticationRequest::new()))
        .await
        .unwrap();

    loop {
        let len = reader.read_i32().await.unwrap_or(0) as usize;
        if len == 0 {
            println!("> {} disconnected", addr);
            break;
        } // TODO Read chunks if more than 1024 bytes!!

        let mut buffer = vec![0u8; len];
        match reader.read_exact(&mut buffer).await {
            Ok(read) => {
                if read == 0 {
                    println!("> {} disconnected", addr);
                    break;
                }

                match ClientMessageType::from(buffer[0]) {
                    ClientMessageType::AuthenticationResponse => {
                        println!("{buffer:?}");
                    }
                    _ => panic!("Received invalid packet"),
                }

                println!("{read}");
            }
            Err(why) => panic!("{why}"),
        }

        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }

    Ok(())
}

async fn keep_alive(sender: Sender<ServerPackets>) {
    println!("Starting KeepAlive thread...");

    let mut interval_timer = tokio::time::interval(Duration::from_secs(5));

    loop {
        interval_timer.tick().await;
        let packet = ServerPackets::KeepAliveRequest(KeepAliveRequest::new());
        println!("KeepAliveRequest sent... {:?}", packet);
        sender.send(packet).await.unwrap();
    }
}
