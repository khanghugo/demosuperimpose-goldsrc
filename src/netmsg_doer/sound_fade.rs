use super::*;

pub struct SoundFade {}
impl<'a> NetMsgDoer<'a, SvcSoundFade> for SoundFade {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSoundFade> {
        map(
            tuple((le_u8, le_u8, le_u8, le_u8)),
            |(initial_percent, hold_time, fade_out_time, fade_in_time)| SvcSoundFade {
                initial_percent,
                hold_time,
                fade_out_time,
                fade_in_time,
            },
        )(i)
    }

    fn write(i: SvcSoundFade) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSoundFade as u8);

        writer.append_u8(i.initial_percent);
        writer.append_u8(i.hold_time);
        writer.append_u8(i.fade_in_time);
        writer.append_u8(i.fade_out_time);

        writer.data
    }
}
