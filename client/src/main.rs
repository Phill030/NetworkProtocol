use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use shared::{
    decoder::ReceiveFromStream,
    messages::{
        client::{AuthenticationResponse, ClientPackets, KeepAliveResponse},
        server::{AuthenticationRequest, KeepAliveRequest, ServerMessageType},
        SystemPacket,
    },
    types::Hwid,
    ADDR, PORT,
};
use std::{
    io::{self, Cursor},
    process,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    spawn,
    sync::mpsc::{channel, Receiver, Sender},
};

const KEY: &str = "HASHING_KEY";

#[tokio::main]
async fn main() -> io::Result<()> {
    if let Ok(stream) = TcpStream::connect(format!("{ADDR}:{PORT}")).await {
        println!("> Connected to server!");

        let (reader, writer) = stream.into_split();
        let (sender, receiver) = channel::<ClientPackets>(100);
        let chat_sender = sender.clone();

        spawn(async move { read_messages(reader, sender).await });
        spawn(async move { write_messages(writer, receiver).await });

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                continue;
            }

            // TODO: Send text
        }
    } else {
        panic!("> Couldn't connect to server...");
    }
}

pub async fn read_messages(mut reader: OwnedReadHalf, sender: Sender<ClientPackets>) {
    let cpu_id = IdBuilder::new(Encryption::SHA256)
        .add_component(HWIDComponent::CPUID)
        .build(KEY)
        .unwrap();
    let system_id = IdBuilder::new(Encryption::SHA256)
        .add_component(HWIDComponent::SystemID)
        .build(KEY)
        .unwrap();

    loop {
        let len = reader.read_i32().await.unwrap_or(0) as usize;
        if len == 0 {
            println!("Server disconnected");
            process::exit(0);
        }

        let mut buffer = vec![0u8; len];
        match reader.read_exact(&mut buffer).await {
            Ok(read) => {
                if read == 0 {
                    println!("Server disconnected");
                    process::exit(0);
                }

                let mut cursor = Cursor::new(buffer);
                match ServerMessageType::from(&mut cursor).await {
                    ServerMessageType::AuthenticationRequest => {
                        let req = AuthenticationRequest::from_bytes(&mut cursor).await.unwrap();
                        println!("Received AuthenticationRequest {req:?}");

                        sender
                            .send(ClientPackets::AuthenticationResponse(AuthenticationResponse {
                                hwid: Hwid {
                                    cpu_id: cpu_id.clone(),
                                    system_id: system_id.clone(),
                                },
                                nonce: req.nonce,
                            }))
                            .await
                            .unwrap();
                    }
                    ServerMessageType::KeepAliveRequest => {
                        let req = KeepAliveRequest::from_bytes(&mut cursor).await.unwrap();
                        println!("Received KeepAliveRequest {req:?}");
                        sender
                            .send(ClientPackets::KeepAliveResponse(KeepAliveResponse { timestamp: req.timestamp }))
                            .await
                            .unwrap();
                    }
                    _ => panic!("Received invalid packet!"),
                }
            }
            Err(why) => panic!("{why}"),
        }
    }
}

pub async fn write_messages(mut writer: OwnedWriteHalf, mut receiver: Receiver<ClientPackets>) {
    while let Some(recv) = receiver.recv().await {
        let buffer = match recv {
            ClientPackets::AuthenticationResponse(x) => x.to_bytes().await,
            ClientPackets::KeepAliveResponse(x) => x.to_bytes().await,
            _ => panic!("Invalid packet received!"),
        }
        .unwrap();

        writer.write_all(&buffer[..]).await.unwrap();
    }
}
