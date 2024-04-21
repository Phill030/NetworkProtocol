use crate::errors::encode::EncodeError;

pub mod client;
pub mod server;

pub trait SystemPacket {
    async fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}

#[macro_export]
macro_rules! message_type {
    ($name:ident; $($variant:ident),*) => {
        use tokio::io::AsyncReadExt;

        #[derive(PartialEq, Debug)]
        pub enum $name {
            $($variant),*,
            InvalidEvent,
        }

        impl $name {
            pub async fn from(cursor: &mut std::io::Cursor<Vec<u8>>) -> Self {
                let value = cursor.read_u8().await.unwrap();

                match value {
                    $(x if x == $name::$variant as u8 => $name::$variant),*,
                    _ => $name::InvalidEvent,
                }
            }
        }

    };
}
