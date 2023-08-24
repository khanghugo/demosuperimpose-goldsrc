use super::*;

pub struct Cutscene {}
impl<'a> NetMsgDoer<'a, SvcCutscene<'a>> for Cutscene {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcCutscene<'a>> {
        map(null_string, |text| SvcCutscene { text })(i)
    }

    fn write(i: SvcCutscene) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcCutscene as u8);

        writer.append_u8_slice(i.text);

        writer.data
    }
}
