use super::*;

pub struct SpawnStatic {}
impl<'a> NetMsgDoer<'a, SvcSpawnStatic> for SpawnStatic {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSpawnStatic> {
        let (
            i,
            (
                model_index,
                sequence,
                frame,
                color_map,
                skin,
                origin_x,
                rotation_x,
                origin_y,
                rotation_y,
                origin_z,
                rotation_z,
                has_render_mode,
            ),
        ) = tuple((
            le_i16, le_i8, le_i8, le_i16, le_i8, le_i16, le_i8, le_i16, le_i8, le_i16, le_i8, le_i8,
        ))(i)?;

        let (i, render_color) = if has_render_mode != 0 {
            map(count(le_u8, 3), |what| Some(what))(i)?
        } else {
            (i, None)
        };

        Ok((
            i,
            SvcSpawnStatic {
                model_index,
                sequence,
                frame,
                color_map,
                skin,
                origin_x,
                rotation_x,
                origin_y,
                rotation_y,
                origin_z,
                rotation_z,
                has_render_mode,
                render_color,
            },
        ))
    }

    fn write(i: SvcSpawnStatic) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSpawnStatic as u8);

        writer.append_i16(i.model_index);
        writer.append_i8(i.sequence);
        writer.append_i8(i.frame);
        writer.append_i16(i.color_map);
        writer.append_i8(i.skin);
        writer.append_i16(i.origin_x);
        writer.append_i8(i.rotation_x);
        writer.append_i16(i.origin_y);
        writer.append_i8(i.rotation_y);
        writer.append_i16(i.origin_z);
        writer.append_i8(i.rotation_z);
        writer.append_i8(i.has_render_mode);

        if i.has_render_mode != 0 {
            for what in i.render_color.unwrap() {
                writer.append_u8(what);
            }
        }

        writer.data
    }
}
