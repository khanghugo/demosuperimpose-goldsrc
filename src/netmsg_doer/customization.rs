use super::*;

pub struct Customization {}
impl<'a> NetMsgDoer<'a, SvcCustomization<'a>> for Customization {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcCustomization<'a>> {
        let (i, (player_index, type_, name, index, download_size, flags)) =
            tuple((le_u8, le_u8, null_string, le_u16, le_u32, le_u8))(i)?;
        let (i, md5_hash) = if flags & 4 != 0 {
            map(take(16usize), |what| Some(what))(i)?
        } else {
            (i, None)
        };

        Ok((
            i,
            SvcCustomization {
                player_index,
                type_,
                name,
                index,
                download_size,
                flags,
                md5_hash,
            },
        ))
    }

    fn write(i: SvcCustomization) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcCustomization as u8);

        writer.append_u8(i.player_index);
        writer.append_u8(i.type_);
        writer.append_u8_slice(i.name);
        writer.append_u16(i.index);
        writer.append_u32(i.download_size);
        writer.append_u8(i.flags);

        if i.flags & 4 != 0 {
            writer.append_u8_slice(i.md5_hash.unwrap());
        }

        writer.data
    }
}
