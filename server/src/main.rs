use shared::{
    decoder::ReceiveFromStream,
    messages::{
        client::{AuthenticationResponse, ClientMessageType, KeepAliveResponse},
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
    const INTERVAL: u64 = 15;

    loop {
        let (stream, addr) = listener.accept().await?;
        let (reader, mut writer) = stream.into_split();
        let (sender, mut receiver) = channel::<ServerPackets>(100);
        let keep_alive_sender = sender.clone();

        spawn(async move { handle_client(addr, reader, sender).await });
        spawn(async move { keep_alive(keep_alive_sender, INTERVAL).await });

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

                let mut cursor = Cursor::new(buffer);
                match ClientMessageType::from(&mut cursor).await {
                    ClientMessageType::AuthenticationResponse => {
                        let res = AuthenticationResponse::from_bytes(&mut cursor).await.unwrap();
                        println!("{res:?}");
                    }

                    ClientMessageType::KeepAliveResponse => {
                        let res = KeepAliveResponse::from_bytes(&mut cursor).await.unwrap();
                        println!("{res:?}");
                    }
                    _ => panic!("Received invalid packet"),
                }

                println!("{read}");
            }
            Err(why) => panic!("{why}"),
        }
    }

    Ok(())
}

async fn keep_alive(sender: Sender<ServerPackets>, interval: u64) {
    println!("Starting KeepAlive thread...");

    let mut interval_timer = tokio::time::interval(Duration::from_secs(interval));

    loop {
        interval_timer.tick().await;
        let packet = ServerPackets::KeepAliveRequest(KeepAliveRequest::new());
        println!("KeepAliveRequest sent... {:?}", packet);
        sender.send(packet).await.unwrap();
    }
}
