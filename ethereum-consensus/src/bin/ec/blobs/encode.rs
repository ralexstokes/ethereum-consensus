use crate::{
    blobs::{
        framing::{sized_header, Mode as Framing},
        Blob, Error, BITS_PER_FIELD_ELEMENT, BYTES_PER_BLOB, BYTES_PER_FIELD_ELEMENT,
    },
    bls::MODULUS,
};
use bitvec::prelude::*;
use ruint::aliases::U256;
use std::io::Read;

type BitSlice = bitvec::slice::BitSlice<u8, bitvec::prelude::Msb0>;

fn field_element_from_bits(src: &BitSlice) -> Result<Vec<u8>, Error> {
    let mut field_element = vec![0u8; BYTES_PER_FIELD_ELEMENT];
    // first two-bits are unusable via the big-endian field element encoding
    let dst = &mut field_element.view_bits_mut()[2..2 + src.len()];
    dst.copy_from_bitslice(src);

    let x = U256::from_be_slice(&field_element);
    if x >= MODULUS {
        Err(Error::ExceedsField)
    } else {
        Ok(field_element)
    }
}

// Pack a buffer of an arbitrary number of bytes into a series of `Blob`s.
pub fn pack_into_blobs(buffer: Vec<u8>) -> Result<Vec<Blob>, Error> {
    let mut blobs = vec![];
    let bits = BitSlice::from_slice(&buffer);
    let mut blob_buffer = Vec::with_capacity(BYTES_PER_BLOB);
    let mut chunks = bits.chunks_exact(BITS_PER_FIELD_ELEMENT);
    for src in chunks.by_ref() {
        if blob_buffer.len() == BYTES_PER_BLOB {
            let blob = Blob::try_from(blob_buffer.as_ref()).expect("is the right size");
            blobs.push(blob);
            blob_buffer.clear();
        }
        let mut field_element = field_element_from_bits(src)?;
        blob_buffer.append(&mut field_element);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        let mut field_element = field_element_from_bits(remainder)?;
        blob_buffer.append(&mut field_element);
    }

    // ensure we have only packed complete field elements so far
    assert!(blob_buffer.len() % BYTES_PER_FIELD_ELEMENT == 0);

    blob_buffer.resize(BYTES_PER_BLOB, 0);
    let blob = Blob::try_from(blob_buffer.as_ref()).expect("is the right size");
    blobs.push(blob);

    Ok(blobs)
}

pub fn from_reader(mut reader: impl Read, framing: Framing) -> Result<Vec<Blob>, Error> {
    let mut buffer = Vec::with_capacity(BYTES_PER_BLOB);

    reader.read_to_end(&mut buffer).expect("can read data");

    let prepared_buffer = if matches!(framing, Framing::Sized) {
        let mut header = sized_header(buffer.len())?;
        let mut framed_buffer = Vec::with_capacity(buffer.len() + header.len());
        framed_buffer.append(&mut header);
        framed_buffer.append(&mut buffer);
        framed_buffer
    } else {
        buffer
    };
    pack_into_blobs(prepared_buffer)
}
