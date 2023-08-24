use super::*;

pub struct SetView {}
impl<'a> NetMsgDoer<'a, SvcSetView> for SetView {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcSetView> {
        map(le_i16, |entity_index| SvcSetView { entity_index })(i)
    }

    fn write(i: SvcSetView) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSetView as u8);

        writer.append_i16(i.entity_index);

        writer.data
    }
}
