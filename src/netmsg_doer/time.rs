use super::*;

pub struct Time {}
impl<'a> NetMsgDoer<'a, SvcTime> for Time {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcTime> {
        map(le_f32, |time| SvcTime { time })(i)
    }

    fn write(i: SvcTime) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(7u8);

        writer.append_f32(i.time);

        writer.data
    }
}
