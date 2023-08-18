use super::*;

pub struct Hltv {}
impl<'a> NetMsgDoer<'a, SvcHltv> for Hltv {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcHltv> {
        map(le_u8, |mode| SvcHltv { mode })(i)
    }

    fn write(i: SvcHltv) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcHltv as u8);

        writer.append_u8(i.mode);

        writer.data
    }
}
