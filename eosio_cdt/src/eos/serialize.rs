use bytes::{Bytes, BytesMut};

pub trait EosSerialize {
    fn write(&self, buf: &mut BytesMut);
    fn read(buf: &mut Bytes) -> Self;
}
