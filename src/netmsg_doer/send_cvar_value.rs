use super::*;

pub struct SendCvarValue {}
impl<'a> NetMsgDoer<'a, SvcSendCvarValue<'a>> for SendCvarValue {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcSendCvarValue<'a>> {
        map(null_string, |name| SvcSendCvarValue { name })(i)
    }

    fn write(i: SvcSendCvarValue) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSendCvarValue as u8);

        writer.append_u8_slice(i.name);

        writer.data
    }
}
