use crate::errors::encode::EncodeError;
use std::future::Future;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

pub trait Encoder {
    fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> impl Future<Output = Result<(), EncodeError>> + Send;
}

pub trait EncoderWriteExt {
    async fn write_string(&mut self, value: &str) -> Result<(), EncodeError>;
}

impl<W: AsyncWrite + Unpin> EncoderWriteExt for W {
    async fn write_string(&mut self, value: &str) -> Result<(), EncodeError> {
        let len = value.len();

        self.write_u32(len.try_into()?).await?;
        self.write_all(value.as_bytes()).await?;

        Ok(())
    }
}

impl Encoder for u8 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u8(*self).await?)
    }
}

impl Encoder for i8 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i8(*self).await?)
    }
}

impl Encoder for u16 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u16(*self).await?)
    }
}

impl Encoder for i16 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i16(*self).await?)
    }
}

impl Encoder for u32 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u32(*self).await?)
    }
}

impl Encoder for i32 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i32(*self).await?)
    }
}

impl Encoder for u64 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u64(*self).await?)
    }
}

impl Encoder for i64 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_i64(*self).await?)
    }
}

impl Encoder for f32 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_f32(*self).await?)
    }
}

impl Encoder for f64 {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_f64(*self).await?)
    }
}

impl Encoder for String {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_string(self).await
    }
}

impl Encoder for &str {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_string(self).await
    }
}

impl Encoder for bool {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let val = match *self {
            true => 1,
            false => 0,
        } as u8;

        Ok(writer.write_u8(val).await?)
    }
}

impl Encoder for Vec<u8> {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let len = self.len();

        writer.write_u32(len.try_into()?).await?;
        Ok(writer.write_all(&self[..]).await?)
    }
}

impl Encoder for Vec<String> {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u32(self.len().try_into()?).await?;

        for val in self {
            writer.write_string(val).await?;
        }

        Ok(())
    }
}

impl<const N: usize> Encoder for [u8; N]
where
    [u8; N]: Sized,
{
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u32(self.len().try_into()?).await?;
        writer.write_all(self).await?;

        Ok(())
    }
}

impl<T: Encoder + Sync> Encoder for Option<T> {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        if let Some(val) = self {
            val.encode(writer).await
        } else {
            0u8.encode(writer).await
        }
    }
}

#[cfg(feature = "uuid")]
impl Encoder for Uuid {
    async fn encode<W: AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
        Ok(writer.write_u128(self.as_u128()).await?)
    }
}
