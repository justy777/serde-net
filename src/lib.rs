mod de;
mod error;
mod ser;

pub use de::{from_bytes, from_reader, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_vec, to_writer, Serializer};
