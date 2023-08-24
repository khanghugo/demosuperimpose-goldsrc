use super::*;

pub struct StopSound {}
impl<'a> NetMsgDoer<'a, SvcStopSound> for StopSound {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcStopSound> {
        map(le_i16, |entity_index| SvcStopSound { entity_index })(i)
    }

    fn write(i: SvcStopSound) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcStopSound as u8);

        writer.append_i16(i.entity_index);

        writer.data
    }
}
