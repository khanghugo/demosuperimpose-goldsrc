use super::*;

pub struct SetAngle {}
impl<'a> NetMsgDoer<'a, SvcSetAngle> for SetAngle {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSetAngle> {
        map(tuple((le_i16, le_i16, le_i16)), |(pitch, yaw, roll)| {
            SvcSetAngle { pitch, yaw, roll }
        })(i)
    }

    fn write(i: SvcSetAngle) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSetAngle as u8);

        writer.append_i16(i.pitch);
        writer.append_i16(i.yaw);
        writer.append_i16(i.roll);

        writer.data
    }
}
