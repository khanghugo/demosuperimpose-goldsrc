use super::*;

pub struct Finale {}
impl<'a> NetMsgDoer<'a, SvcFinale<'a>> for Finale {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcFinale<'a>> {
        map(null_string, |text| SvcFinale { text })(i)
    }

    fn write(i: SvcFinale) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcFinale as u8);

        writer.append_u8_slice(i.text);

        writer.data
    }
}
