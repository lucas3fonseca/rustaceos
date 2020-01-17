use bytes::Bytes;

pub trait AbiWrite {
    fn write(&self) -> Vec<u8>;
}

pub trait AbiRead {
    fn read(buf: &mut Bytes) -> Self;
}
