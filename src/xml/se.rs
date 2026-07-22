//! Module to handle custom serde `Serializer`
//!
//! This module serves as a patch for the quick_xml serde support.

use quick_xml::{SeError, DeError};
use serde::ser::Serialize;

#[cfg(not(feature = "binary"))]
struct ByteWriter<W>(W)
where
    W: std::io::Write;

#[cfg(not(feature = "binary"))]
impl<W: std::io::Write> std::fmt::Write for ByteWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write(s.as_bytes()).map_err(|_| std::fmt::Error)?;
        std::fmt::Result::Ok(())
    }
}

#[cfg(not(feature = "binary"))]
impl<W: std::io::Write> std::io::Write for ByteWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

/// Serialize struct into an  `io::Write`r
#[cfg(feature = "binary")]
pub fn to_writer<W: std::io::Write, S: Serialize>(writer: W, value: &S) -> Result<(), SeError> {
    let mut buf_writer = std::io::BufWriter::new(writer);
    let serializer = quick_xml::se::io::Serializer::new(&mut buf_writer);
    value.serialize(serializer)?;
    Ok(())
}

#[cfg(not(feature = "binary"))]
pub fn to_writer<W: std::io::Write, S: Serialize>(writer: W, value: &S) -> Result<(), SeError> {
    let mut buf_writer = ByteWriter(std::io::BufWriter::new(writer));
    let serializer = quick_xml::se::Serializer::new(&mut buf_writer);
    value.serialize(serializer)?;
    Ok(())
}

/// Serialize struct into a `fmt::Write`r
pub fn to_fmt_writer<W: std::fmt::Write, S: Serialize>(
    mut writer: W,
    value: &S,
) -> Result<(), SeError> {
    let serializer = quick_xml::se::Serializer::new(&mut writer);
    value.serialize(serializer)?;
    Ok(())
}

/// Serialize struct into a `String`
pub fn to_string<S: Serialize>(value: &S) -> Result<String, SeError> {
    let mut s = String::new();
    let serializer = quick_xml::se::Serializer::new(&mut s);
    value.serialize(serializer)?;
    Ok(s)
}

/// Serialize struct into a `Vec<u8>`
pub fn to_bytes<S: Serialize>(value: &S) -> Result<Vec<u8>, SeError> {
    let mut bytes = Vec::new();
    to_writer(&mut bytes, value)?;
    Ok(bytes)
}
