use super::*;

pub struct SendExtraInfo {}
impl<'a> NetMsgDoer<'a, SvcSendExtraInfo<'a>> for SendExtraInfo {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSendExtraInfo<'a>> {
        map(tuple((null_string, le_u8)), |(fallback_dir, can_cheat)| {
            SvcSendExtraInfo {
                fallback_dir,
                can_cheat,
            }
        })(i)
    }

    fn write(i: SvcSendExtraInfo<'a>) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(54u8);

        writer.append_u8_slice(i.fallback_dir);
        writer.append_u8(i.can_cheat);

        writer.data
    }
}
