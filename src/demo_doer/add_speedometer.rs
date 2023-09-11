use crate::wrap_message;

use super::*;

pub fn add_speed_o_meter(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    let mut last_time: Option<f32> = None;
    let mut last_pos: Option<[f32; 3]> = None;
    let mut speed = 0.;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            match &mut frame.data {
                FrameData::ClientData(client_data) => {
                    if last_pos.is_none() {
                        last_pos = Some(client_data.origin);
                        last_time = Some(frame.time);
                        continue;
                    }

                    // SAFETY: values are guaranteed to be None here.
                    let time_delta = frame.time - last_time.unwrap();

                    if time_delta != 0. {
                        let abs_displacement = (0..2)
                            .map(|i| client_data.origin[i] - last_pos.unwrap()[i])
                            .fold(0., |acc, num| num * num + acc)
                            .sqrt();
                        speed = abs_displacement / time_delta;
                    }

                    last_pos = Some(client_data.origin);
                    last_time = Some(frame.time);
                }
                FrameData::NetMsg((_, data)) => {
                    let (_, mut messages) =
                        parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                    let message = format!("{:.1}\0", speed);
                    let message = message.as_bytes();

                    let text = TeTextMessage {
                        channel: 4,
                        // (0, 0) is top left
                        x: 0.48f32.coord_conversion(),
                        y: 0.75f32.coord_conversion(),
                        effect: 0,
                        text_color: &[255, 255, 255, 0],
                        effect_color: &[255, 255, 255, 0],
                        fade_in_time: 25,
                        fade_out_time: 76,
                        hold_time: 60,
                        effect_time: None,
                        message,
                    };

                    let temp_entity = SvcTempEntity {
                        entity_type: 29,
                        entity: TempEntityEntity::TeTextMessage(text),
                    };

                    messages.push(wrap_message!(SvcTempEntity, temp_entity));

                    let write = write_netmsg(messages, &mut delta_decoders, &custom_messages);

                    data.msg = write.leak();
                }
                _ => (),
            };
        }
    }
}

trait CoordConversion {
    fn coord_conversion(&self) -> i16;
}

impl CoordConversion for f32 {
    fn coord_conversion(&self) -> i16 {
        (self * 8192.).round() as i16
    }
}
