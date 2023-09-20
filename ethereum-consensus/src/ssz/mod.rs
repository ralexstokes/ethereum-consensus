mod byte_list;
mod byte_vector;

use std::fmt;

fn write_bytes_to_lower_hex<T: AsRef<[u8]>>(f: &mut fmt::Formatter<'_>, data: T) -> fmt::Result {
    if f.alternate() {
        write!(f, "0x")?;
    }
    for i in data.as_ref() {
        write!(f, "{i:02x}")?;
    }
    Ok(())
}

pub mod prelude {
    pub use super::{byte_list::ByteList, byte_vector::ByteVector};
    pub use ssz_rs::prelude::*;
}
