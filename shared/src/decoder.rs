use crate::errors::decode::DecodeError;
use std::{io::Cursor, mem::size_of, ops::Mul};
use tokio::io::{AsyncRead, AsyncReadExt};
use uuid::Uuid;

pub trait Decoder {
    type Output;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError>;
}

pub trait ReceiveFromStream: Sized {
    async fn from_bytes(buffer: &mut Cursor<Vec<u8>>) -> Result<Self, DecodeError>;
}

pub trait DecoderReadExt {
    async fn read_string(&mut self) -> Result<String, DecodeError>;
    async fn read_byte_array(&mut self) -> Result<Vec<u8>, DecodeError>;
}

impl<R: AsyncRead + Unpin> DecoderReadExt for R {
    async fn read_byte_array(&mut self) -> Result<Vec<u8>, DecodeError> {
        let length = self.read_u32().await?;

        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf).await?;

        Ok(buf)
    }

    async fn read_string(&mut self) -> Result<String, DecodeError> {
        let len = self.read_u32().await? as usize;

        let mut buf = vec![0; len];
        self.read_exact(&mut buf).await?;

        Ok(String::from_utf8(buf)?)
    }
}

impl Decoder for u8 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u8().await?)
    }
}

impl Decoder for i8 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i8().await?)
    }
}

impl Decoder for i16 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i16().await?)
    }
}

impl Decoder for u16 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u16().await?)
    }
}

impl Decoder for i32 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i32().await?)
    }
}

impl Decoder for u32 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u32().await?)
    }
}

impl Decoder for f32 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_f32().await?)
    }
}

impl Decoder for f64 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_f64().await?)
    }
}

impl Decoder for u64 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_u64().await?)
    }
}

impl Decoder for i64 {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        Ok(reader.read_i64().await?)
    }
}

impl Decoder for String {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        reader.read_string().await
    }
}

impl Decoder for bool {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        match reader.read_u8().await? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::NonBoolValue),
        }
    }
}

impl<T: Decoder<Output = T>> Decoder for Vec<T> {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        let len = reader.read_u32().await?;

        let mut x_vec: Vec<T> = Vec::with_capacity((len as usize).mul(size_of::<T>()));
        for _ in 0..len {
            x_vec.push(T::decode(reader).await?);
        }

        Ok(x_vec)
    }
}

#[cfg(feature = "uuid")]
impl Decoder for Uuid {
    type Output = Self;

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
        let val = reader.read_u128().await?;
        Ok(Uuid::from_u128(val))
    }
}
