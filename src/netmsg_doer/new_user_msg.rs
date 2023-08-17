use super::*;

pub struct NewUserMsg {}
impl<'a> NetMsgDoer<'a, SvcNewUserMsg<'a>> for NewUserMsg {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcNewUserMsg<'a>> {
        map(
            tuple((le_u8, le_u8, take(16usize))),
            |(index, size, name)| SvcNewUserMsg { index, size, name },
        )(i)
    }

    fn write(i: SvcNewUserMsg) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcNewUserMsg as u8);

        writer.append_u8(i.index);
        writer.append_u8(i.size);
        writer.append_u8_slice(i.name);

        writer.data
    }
}
