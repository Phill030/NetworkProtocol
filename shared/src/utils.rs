use crate::errors::encode::EncodeError;
use std::mem::size_of;
use tokio::io::AsyncWriteExt;

pub async fn prepare_response(event_id: u8, mut data: Vec<u8>) -> Result<Vec<u8>, EncodeError> {
    let mut buffer = vec![];
    buffer.write_u32(u32::try_from(size_of::<u8>() + data.len())?).await?;
    buffer.write_u8(event_id).await?;
    buffer.append(&mut data);

    Ok(buffer)
}
