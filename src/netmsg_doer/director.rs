use super::*;

pub struct Director {}
impl<'a> NetMsgDoer<'a, SvcDirector<'a>> for Director {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcDirector<'a>> {
        map(
            tuple((le_u8, le_u8, null_string)),
            |(length, flag, message)| SvcDirector {
                length,
                flag,
                message,
            },
        )(i)
    }

    fn write(i: SvcDirector) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcDirector as u8);

        writer.append_u8(i.length);
        writer.append_u8(i.flag);
        writer.append_u8_slice(i.message);

        writer.data
    }
}
