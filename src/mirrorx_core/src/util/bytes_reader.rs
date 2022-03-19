use bytes::Buf;

pub struct BytesReader<'a> {
    buf: &'a [u8],
}

impl<'a> BytesReader<'a> {
    pub fn new(buf: &[u8]) -> BytesReader<'_> {
        BytesReader { buf }
    }

    pub fn read_bool(&mut self) -> anyhow::Result<bool> {
        if self.buf.remaining() < 1 {
            return Err(anyhow::anyhow!("reader has no enough bytes to read bool"));
        }

        Ok(self.buf.get_u8() == 0x01)
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        if self.buf.remaining() < 1 {
            return Err(anyhow::anyhow!("reader has no enough bytes to read u8"));
        }

        Ok(self.buf.get_u8())
    }

    pub fn read_u16(&mut self) -> anyhow::Result<u16> {
        if self.buf.remaining() < 2 {
            return Err(anyhow::anyhow!("reader has no enough bytes to read u16"));
        }

        Ok(self.buf.get_u16_le())
    }

    pub fn read_u32(&mut self) -> anyhow::Result<u32> {
        if self.buf.remaining() < 4 {
            return Err(anyhow::anyhow!("reader has no enough bytes to read u32"));
        }

        Ok(self.buf.get_u32_le())
    }

    pub fn read_u64(&mut self) -> anyhow::Result<u64> {
        if self.buf.remaining() < 8 {
            return Err(anyhow::anyhow!("reader has no enough bytes to read u64"));
        }

        Ok(self.buf.get_u64_le())
    }

    pub fn read_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        let body_length = self.read_u16()? as usize;

        if self.buf.remaining() < body_length {
            return Err(anyhow::anyhow!(
                "reader has no enough bytes to read bytes body"
            ));
        }

        let body = match self.buf.get(0..body_length) {
            Some(res) => res,
            None => {
                return Err(anyhow::anyhow!(
                    "reader has no enough bytes to read bytes body"
                ))
            }
        };

        Ok(body.to_vec())
    }

    pub fn read_string(&mut self) -> anyhow::Result<String> {
        let body = self.read_bytes()?;
        String::from_utf8(body).or_else(|err| Err(anyhow::anyhow!(err)))
    }

    pub fn read_remaining_bytes(&mut self) -> anyhow::Result<&'a [u8]> {
        match self.buf.get(..) {
            Some(res) => Ok(res),
            None => Err(anyhow::anyhow!("reader has no remaining bytes")),
        }
    }
}
