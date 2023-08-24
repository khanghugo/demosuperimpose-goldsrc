use super::*;

pub struct AddAngle {}
impl<'a> NetMsgDoer<'a, SvcAddAngle> for AddAngle {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcAddAngle> {
        map(le_i16, |angle_to_add| SvcAddAngle { angle_to_add })(i)
    }

    fn write(i: SvcAddAngle) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcAddAngle as u8);

        writer.append_i16(i.angle_to_add);

        writer.data
    }
}
