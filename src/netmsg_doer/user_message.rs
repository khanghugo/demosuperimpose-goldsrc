use super::*;

pub struct UserMessage {}
impl<'a> NetMsgDoer<'a, NetMsgUserMessage<'a>> for UserMessage {
    fn parse(i: &'a [u8]) -> IResult<&[u8], NetMsgUserMessage> {
        let (i, length) = le_u8(i)?;
        map(take(length), |x| NetMsgUserMessage { message: x })(i)
    }

    fn write(i: NetMsgUserMessage) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(i.message.len() as u8);
        writer.append_u8_slice(i.message);

        writer.data
    }
}
