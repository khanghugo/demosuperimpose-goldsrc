use super::*;

pub struct SendCvarValue2 {}
impl<'a> NetMsgDoer<'a, SvcSendCvarValue2<'a>> for SendCvarValue2 {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcSendCvarValue2<'a>> {
        map(tuple((le_u32, null_string)), |(request_id, name)| {
            SvcSendCvarValue2 { request_id, name }
        })(i)
    }

    fn write(i: SvcSendCvarValue2) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSendCvarValue2 as u8);

        writer.append_u32(i.request_id);
        writer.append_u8_slice(i.name);

        writer.data
    }
}
