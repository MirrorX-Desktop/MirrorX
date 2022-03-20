use bytes::{BufMut, BytesMut};

pub struct BytesWriter<'a> {
    buf: &'a mut BytesMut,
}

impl BytesWriter<'_> {
    pub fn new(buf: &mut BytesMut) -> BytesWriter<'_> {
        BytesWriter { buf }
    }

    pub fn write_bool(&mut self, value: bool) {
        if value {
            self.buf.put_u8(0x01)
        } else {
            self.buf.put_u8(0x00)
        }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buf.put_u8(value)
    }

    pub fn write_u16(&mut self, value: u16) {
        self.buf.put_u16_le(value)
    }

    pub fn write_u32(&mut self, value: u32) {
        self.buf.put_u32_le(value)
    }

    pub fn write_u64(&mut self, value: u64) {
        self.buf.put_u64_le(value)
    }

    pub fn write_bytes(&mut self, value: &[u8]) {
        self.buf.put_u16_le(value.len() as u16);
        self.buf.put_slice(value);
    }

    pub fn write_string(&mut self, value: &String) {
        let value_bytes = value.as_bytes();
        self.write_bytes(value_bytes);
    }
}
