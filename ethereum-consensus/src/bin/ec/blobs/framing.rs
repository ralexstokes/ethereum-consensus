use crate::blobs::Error;

pub const FRAMING_VERSION: u8 = 0;

pub enum Mode {
    Raw,
    Sized,
}

impl TryFrom<String> for Mode {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "raw" => Ok(Self::Raw),
            "sized" => Ok(Self::Sized),
            other => Err(Error::InvalidFrameMode(other.into())),
        }
    }
}

pub fn sized_header(data_byte_length: usize) -> Result<Vec<u8>, Error> {
    let mut header = vec![0u8; 5];
    header[0] = FRAMING_VERSION;
    let size = u32::try_from(data_byte_length)
        .map_err(|_| Error::ExceedsMaxFrameSize(data_byte_length))?;
    header[1..].copy_from_slice(&size.to_be_bytes());
    Ok(header)
}

pub fn payload_from_sized(stream: Vec<u8>) -> Vec<u8> {
    assert!(stream.len() >= 5);
    let (header, payload) = stream.split_at(5);
    assert!(header[0] == FRAMING_VERSION);
    let size = u32::from_be_bytes(header[1..5].try_into().expect("correct size bytes")) as usize;
    payload[..size].to_vec()
}
