use super::*;

pub struct Director {}
impl<'a> NetMsgDoer<'a, SvcDirector<'a>> for Director {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcDirector<'a>> {
        let (i, (length, flag)) = tuple((le_u8, le_u8))(i)?;
        let (i, message) = take(length - 1)(i)?;

        Ok((
            i,
            SvcDirector {
                length,
                flag,
                message,
            },
        ))
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
