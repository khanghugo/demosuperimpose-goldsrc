use super::*;

pub struct TimeScale {}
impl<'a> NetMsgDoer<'a, SvcTimeScale> for TimeScale {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcTimeScale> {
        map(le_f32, |time_scale| SvcTimeScale { time_scale })(i)
    }

    fn write(i: SvcTimeScale) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcTimeScale as u8);

        writer.append_f32(i.time_scale);

        writer.data
    }
}
