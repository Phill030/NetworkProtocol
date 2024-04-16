use crate::errors::encode::EncodeError;

pub mod client;
pub mod server;

pub trait SystemPacket {
    async fn to_bytes(&self) -> Result<Vec<u8>, EncodeError>;
}

#[macro_export]
macro_rules! message_type {
    ($name:ident; $($variant:ident),*) => {
        #[derive(PartialEq, Debug)]
        pub enum $name {
            $($variant),*,
            InvalidEvent,
        }

        impl From<u8> for $name {
            fn from(value: u8) -> Self {
                match value {
                    $(x if x == $name::$variant as u8 => $name::$variant),*,
                    _ => $name::InvalidEvent,
                }
            }
        }
    };
}
