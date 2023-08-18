use super::*;

pub struct VoiceData {}
impl<'a> NetMsgDoer<'a, SvcVoiceData<'a>> for VoiceData {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcVoiceData<'a>> {
        let (i, (player_index, size)) = tuple((le_u8, le_u16))(i)?;
        let (i, data) = take(player_index)(i)?;

        Ok((
            i,
            SvcVoiceData {
                player_index,
                size,
                data,
            },
        ))
    }

    fn write(i: SvcVoiceData) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcVoiceData as u8);

        writer.append_u8(i.player_index);
        writer.append_u16(i.size);
        writer.append_u8_slice(i.data);

        writer.data
    }
}
