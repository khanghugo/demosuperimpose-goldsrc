use super::*;

pub struct DecalName {}
impl<'a> NetMsgDoer<'a, SvcDecalName<'a>> for DecalName {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcDecalName<'a>> {
        map(
            tuple((le_u8, null_string)),
            |(position_index, decal_name)| SvcDecalName {
                position_index,
                decal_name,
            },
        )(i)
    }

    fn write(i: SvcDecalName) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcDecalName as u8);

        writer.append_u8(i.position_index);
        writer.append_u8_slice(i.decal_name);

        writer.data
    }
}
