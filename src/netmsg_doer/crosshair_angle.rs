use super::*;

pub struct CrosshairAngle {}
impl<'a> NetMsgDoer<'a, SvcCrosshairAngle> for CrosshairAngle {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcCrosshairAngle> {
        map(tuple((le_i16, le_i16)), |(pitch, yaw)| SvcCrosshairAngle {
            pitch,
            yaw,
        })(i)
    }

    fn write(i: SvcCrosshairAngle) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcCrosshairAngle as u8);

        writer.append_i16(i.pitch);
        writer.append_i16(i.yaw);

        writer.data
    }
}
