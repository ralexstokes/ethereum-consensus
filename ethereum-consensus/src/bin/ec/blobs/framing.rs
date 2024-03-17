use crate::blobs::Error;

pub const SIZED_FRAMING_VERSION: u8 = 0;
pub const HEADER_SIZE: usize = 5;

/// A `Mode` to indicate how the target data should be packed into blob data.
pub enum Mode {
    /// No framing, data is written/read directly from the blob data
    Raw,
    /// The size of a "payload" is written in-band to the blob data.
    /// Supports "lossless" {de,}serialization if the payload data is not
    /// a multiple of the blob size.
    Sized,
}

// Returns the header bytes that should prepend the target data in `Sized` framing mode.
// The header consists of one version byte, then a `u32` integer in big-endian encoding containing
// the size of the trailing data.
pub fn sized_header(data_byte_length: usize) -> Result<[u8; HEADER_SIZE], Error> {
    let mut header = [0u8; HEADER_SIZE];
    header[0] = SIZED_FRAMING_VERSION;
    let size = u32::try_from(data_byte_length).map_err(|_| Error::InvalidPayloadSize)?;
    header[1..].copy_from_slice(&size.to_be_bytes());
    Ok(header)
}

// Attempts to parse a `stream` of bytes assuming they were written to blobs with the `Sized`
// framing mode.
pub fn payload_from_sized(stream: &[u8]) -> Result<&[u8], Error> {
    if stream.len() < HEADER_SIZE {
        return Err(Error::ExpectedHeaderForSizedFraming)
    }

    let (header, payload) = stream.split_at(HEADER_SIZE);

    if header[0] != SIZED_FRAMING_VERSION {
        return Err(Error::UnsupportedSizedFramingVersion)
    }
    let size = u32::from_be_bytes(header[1..5].try_into().expect("correct size bytes")) as usize;
    if size >= stream.len() {
        return Err(Error::InvalidPayloadSize)
    }

    Ok(&payload[..size])
}
