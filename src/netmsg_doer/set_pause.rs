use super::*;

pub struct SetPause {}
impl<'a> NetMsgDoer<'a, SvcSetPause> for SetPause {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSetPause> {
        map(le_i8, |is_paused| SvcSetPause { is_paused })(i)
    }

    fn write(i: SvcSetPause) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSetPause as u8);

        writer.append_i8(i.is_paused);

        writer.data
    }
}
