use super::*;

pub struct Restore {}
impl<'a> NetMsgDoer<'a, SvcRestore<'a>> for Restore {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcRestore<'a>> {
        let (i, (save_name, map_count)) = tuple((null_string, le_u8))(i)?;
        let (i, map_names) = count(null_string, map_count as usize)(i)?;

        Ok((
            i,
            SvcRestore {
                save_name,
                map_count,
                map_names,
            },
        ))
    }

    fn write(i: SvcRestore) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcRestore as u8);

        writer.append_u8_slice(i.save_name);
        writer.append_u8(i.map_count);
        for what in i.map_names {
            writer.append_u8_slice(what);
        }

        writer.data
    }
}
