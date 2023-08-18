use super::*;

pub struct SpawnStaticSound {}
impl<'a> NetMsgDoer<'a, SvcSpawnStaticSound> for SpawnStaticSound {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSpawnStaticSound> {
        map(
            tuple((count(le_i16, 3), le_u16, le_u8, le_u8, le_u16, le_u8, le_u8)),
            |(origin, sound_index, volume, attenuation, entity_index, pitch, flags)| {
                SvcSpawnStaticSound {
                    origin,
                    sound_index,
                    volume,
                    attenuation,
                    entity_index,
                    pitch,
                    flags,
                }
            },
        )(i)
    }

    fn write(i: SvcSpawnStaticSound) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSpawnStaticSound as u8);

        for what in i.origin {
            writer.append_i16(what)
        }
        writer.append_u16(i.sound_index);
        writer.append_u8(i.volume);
        writer.append_u8(i.attenuation);
        writer.append_u16(i.entity_index);
        writer.append_u8(i.pitch);
        writer.append_u8(i.flags);

        writer.data
    }
}
