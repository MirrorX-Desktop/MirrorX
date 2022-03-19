use crate::util::{BytesReader, BytesWriter};

use super::{opcode::Opcode, ProtoMessage};

#[derive(Debug)]
pub struct HeartBeatReq {
    pub time_stamp: u32,
}

impl ProtoMessage for HeartBeatReq {
    fn opcode(&self) -> u16 {
        Opcode::HeartBeatReq.into()
    }

    fn encode(&self, writer: &mut BytesWriter) {
        writer.write_u32(self.time_stamp);
    }

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        let mut reader = BytesReader::new(&buf);
        self.time_stamp = reader.read_u32()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct HeartBeatResp {
    pub time_stamp: u32,
}

impl ProtoMessage for HeartBeatResp {
    fn opcode(&self) -> u16 {
        Opcode::HeartBeatResp.into()
    }

    fn encode(&self, writer: &mut BytesWriter) {
        writer.write_u32(self.time_stamp);
    }

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        let mut reader = BytesReader::new(&buf);
        self.time_stamp = reader.read_u32()?;
        Ok(())
    }
}
