use super::*;

pub struct LightStyle {}
impl<'a> NetMsgDoer<'a, SvcLightStyle<'a>> for LightStyle {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcLightStyle<'a>> {
        map(tuple((le_u8, null_string)), |(index, light_info)| {
            SvcLightStyle { index, light_info }
        })(i)
    }

    fn write(i: SvcLightStyle) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcLightStyle as u8);

        writer.append_u8(i.index);
        writer.append_u8_slice(i.light_info);

        writer.data
    }
}
