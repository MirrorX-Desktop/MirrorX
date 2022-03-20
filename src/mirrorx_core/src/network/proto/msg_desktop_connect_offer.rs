use super::{opcode::Opcode, ProtoMessage};
use crate::network::util::{BytesReader, BytesWriter};

#[derive(Debug)]
pub struct DesktopConnectOfferReq {
    pub device_id: String,
}

impl ProtoMessage for DesktopConnectOfferReq {
    fn opcode(&self) -> u16 {
        Opcode::DesktopConnectOfferReq.into()
    }

    fn encode(&self, writer: &mut BytesWriter) {
        writer.write_string(&self.device_id)
    }

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        let mut reader = BytesReader::new(&buf);
        self.device_id = reader.read_string()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DesktopConnectOfferResp {
    pub allow: bool,
}

impl ProtoMessage for DesktopConnectOfferResp {
    fn opcode(&self) -> u16 {
        Opcode::DesktopConnectOfferResp.into()
    }

    fn encode(&self, writer: &mut BytesWriter) {
        writer.write_bool(self.allow)
    }

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()> {
        let mut reader = BytesReader::new(&buf);
        self.allow = reader.read_bool()?;
        Ok(())
    }
}
