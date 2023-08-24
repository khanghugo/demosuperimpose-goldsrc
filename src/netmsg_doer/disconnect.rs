use super::*;

pub struct Disconnect {}
impl<'a> NetMsgDoer<'a, SvcDisconnect<'a>> for Disconnect {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcDisconnect<'a>> {
        map(null_string, |reason| SvcDisconnect { reason })(i)
    }

    fn write(i: SvcDisconnect) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcDisconnect as u8);

        writer.append_u8_slice(i.reason);

        writer.data
    }
}
