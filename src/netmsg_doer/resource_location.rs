use super::*;

pub struct ResourceLocation {}
impl<'a> NetMsgDoer<'a, SvcResourceLocation<'a>> for ResourceLocation {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcResourceLocation<'a>> {
        map(null_string, |download_url| SvcResourceLocation {
            download_url,
        })(i)
    }

    fn write(i: SvcResourceLocation) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcResourceLocation as u8);

        writer.append_u8_slice(i.download_url);

        writer.data
    }
}
