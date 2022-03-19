use super::{opcode::Opcode, ProtoMessage};
use crate::util::{BytesReader, BytesWriter};

#[derive(Debug)]
pub struct DesktopConnectAskReq {
    pub offer_device_id: String,
}

impl ProtoMessage for DesktopConnectAskReq {
    fn opcode(&self) -> u16 {
        Opcode::DesktopConnectAskReq.into()
    }

    fn encode(&self, writer: &mut BytesWriter) {
        writer.write_string(&self.offer_device_id)
    }

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        let mut reader = BytesReader::new(&buf);
        self.offer_device_id = reader.read_string()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DesktopConnectAskResp {
    pub agree: bool,
}

impl ProtoMessage for DesktopConnectAskResp {
    fn opcode(&self) -> u16 {
        Opcode::DesktopConnectAskResp.into()
    }

    fn encode(&self, writer: &mut BytesWriter) {
        writer.write_bool(self.agree)
    }

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        let mut reader = BytesReader::new(&buf);
        self.agree = reader.read_bool()?;
        Ok(())
    }
}
