use super::*;

pub struct FileTxferFailed {}
impl<'a> NetMsgDoer<'a, SvcFileTxferFailed<'a>> for FileTxferFailed {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcFileTxferFailed<'a>> {
        map(null_string, |file_name| SvcFileTxferFailed { file_name })(i)
    }

    fn write(i: SvcFileTxferFailed) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcFileTxferFailed as u8);

        writer.append_u8_slice(i.file_name);

        writer.data
    }
}
