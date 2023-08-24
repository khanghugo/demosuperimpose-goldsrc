use super::*;

pub struct Print {}
impl<'a> NetMsgDoer<'a, SvcPrint<'a>> for Print {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcPrint<'a>> {
        map(null_string, |message| SvcPrint { message })(i)
    }

    fn write(i: SvcPrint<'a>) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcPrint as u8);

        writer.append_u8_slice(i.message);

        writer.data
    }
}
