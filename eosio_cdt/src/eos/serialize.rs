use bytes::{Bytes, BytesMut};

pub use serde::{Deserialize, Serialize};

pub use bincode::deserialize as eos_deserialize;
pub use bincode::serialize as eos_serialize;

pub trait EosSerialize {
    fn write(&self, buf: &mut BytesMut);
    fn read(buf: &mut Bytes) -> Self;
}
