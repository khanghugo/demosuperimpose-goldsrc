use super::*;

pub struct Version {}
impl<'a> NetMsgDoer<'a, SvcVersion> for Version {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcVersion> {
        map(le_u32, |protocol_version| SvcVersion { protocol_version })(i)
    }

    fn write(i: SvcVersion) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcVersion as u8);

        writer.append_u32(i.protocol_version);

        writer.data
    }
}
