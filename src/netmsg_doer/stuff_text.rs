use super::*;

pub struct StuffText {}
impl<'a> NetMsgDoer<'a, SvcStuffText<'a>> for StuffText {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcStuffText<'a>> {
        map(null_string, |command| SvcStuffText { command })(i)
    }

    fn write(i: SvcStuffText) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcStuffText as u8);

        writer.append_u8_slice(i.command);

        writer.data
    }
}
