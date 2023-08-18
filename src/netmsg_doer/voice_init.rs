use super::*;

pub struct VoiceInit {}
impl<'a> NetMsgDoer<'a, SvcVoiceInit<'a>> for VoiceInit {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcVoiceInit<'a>> {
        map(tuple((null_string, le_i8)), |(codec_name, quality)| {
            SvcVoiceInit {
                codec_name,
                quality,
            }
        })(i)
    }

    fn write(i: SvcVoiceInit) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcVoiceInit as u8);

        writer.append_u8_slice(i.codec_name);
        writer.append_i8(i.quality);

        writer.data
    }
}
