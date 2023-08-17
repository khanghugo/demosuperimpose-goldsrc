use super::*;

pub struct CenterPrint {}
impl<'a> NetMsgDoer<'a, SvcCenterPrint<'a>> for CenterPrint {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcCenterPrint<'a>> {
        map(null_string, |message| SvcCenterPrint { message })(i)
    }

    fn write(i: SvcCenterPrint) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcCenterPrint as u8);

        writer.append_u8_slice(i.message);

        writer.data
    }
}
