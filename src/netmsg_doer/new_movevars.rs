use super::*;

pub struct NewMovevars {}
impl<'a> NetMsgDoer<'a, SvcNewMoveVars<'a>> for NewMovevars {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcNewMoveVars<'a>> {
        // https://github.com/rust-bakery/nom/issues/1144
        map(
            tuple((
                tuple((
                    le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32,
                    le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_i32, le_f32, le_f32,
                )),
                count(le_f32, 3),
                count(le_f32, 3),
                null_string,
            )),
            |
            (
                (gravity,
                stop_speed,
                max_speed,
                spectator_max_speed,
                accelerate,
                airaccelerate,
                water_accelerate,
                friction,
                edge_friction,
                water_friction,
                ent_garvity,
                bounce,
                step_size,
                max_velocity,
                z_max,
                wave_height,
                footsteps,
                roll_angle,
                roll_speed),
                sky_color,
                sky_vec,
                sky_name,
            )
            // what
            | SvcNewMoveVars {
                gravity,
                stop_speed,
                max_speed,
                spectator_max_speed,
                accelerate,
                airaccelerate,
                water_accelerate,
                friction,
                edge_friction,
                water_friction,
                ent_garvity,
                bounce,
                step_size,
                max_velocity,
                z_max,
                wave_height,
                footsteps,
                roll_angle,
                roll_speed,
                sky_color,
                sky_vec,
                sky_name,
            },
        )(i)
    }

    fn write(i: SvcNewMoveVars<'a>) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcNewMoveVars as u8);

        writer.append_f32(i.gravity);
        writer.append_f32(i.stop_speed);
        writer.append_f32(i.max_speed);
        writer.append_f32(i.spectator_max_speed);
        writer.append_f32(i.accelerate);
        writer.append_f32(i.airaccelerate);
        writer.append_f32(i.water_accelerate);
        writer.append_f32(i.friction);
        writer.append_f32(i.edge_friction);
        writer.append_f32(i.water_friction);
        writer.append_f32(i.ent_garvity);
        writer.append_f32(i.bounce);
        writer.append_f32(i.step_size);
        writer.append_f32(i.max_velocity);
        writer.append_f32(i.z_max);
        writer.append_f32(i.wave_height);
        writer.append_i32(i.footsteps);
        writer.append_f32(i.roll_angle);
        writer.append_f32(i.roll_speed);
        for e in i.sky_color {
            writer.append_f32(e);
        }
        for e in i.sky_vec {
            writer.append_f32(e);
        }
        writer.append_u8_slice(i.sky_name);

        writer.data
    }
}
