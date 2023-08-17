use super::*;

pub struct SignOnNum {}
impl<'a> NetMsgDoer<'a, SvcSignOnNum> for SignOnNum {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSignOnNum> {
        map(le_i8, |sign| SvcSignOnNum { sign })(i)
    }

    fn write(i: SvcSignOnNum) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSignOnNum as u8);

        writer.append_i8(i.sign);

        writer.data
    }
}
