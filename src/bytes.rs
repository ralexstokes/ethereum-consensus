use std::fmt;

pub fn write_bytes_to_lower_hex<T: AsRef<[u8]>>(
    f: &mut fmt::Formatter<'_>,
    data: T,
) -> fmt::Result {
    if f.alternate() {
        write!(f, "0x")?;
    }
    for i in data.as_ref() {
        write!(f, "{:02x}", i)?;
    }
    Ok(())
}
