use crate::init_parse;
use crate::wrap_message;

use super::*;

use demosuperimpose_goldsrc::netmsg_doer::parse_netmsg_immutable;
use rayon::prelude::*;

/// Adds entry index and frame_index on screen.
pub fn add_debug(demo: &mut Demo) {
    let (delta_decoders, custom_messages) = init_parse!(demo);

    for (entry_idx, entry) in demo.directory.entries.iter_mut().skip(1).enumerate() {
        entry
            .frames
            .par_iter_mut()
            .enumerate()
            .for_each(|(frame_idx, frame)| {
                match &mut frame.data {
                    FrameData::NetMsg((_, data)) => {
                        let (_, mut messages) =
                            parse_netmsg_immutable(data.msg, &delta_decoders, &custom_messages)
                                .unwrap();

                        let message = format!(
                            "{} {} \n {} {}\0",
                            entry_idx,
                            frame_idx,
                            data.info.ref_params.viewangles[0],
                            data.info.ref_params.viewangles[1]
                        );
                        let message = message.as_bytes();

                        let text = TeTextMessage {
                            channel: 4,
                            // (0, 0) is top left
                            x: 0.48f32.coord_conversion(),
                            y: 0.50f32.coord_conversion(),
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

                        let write = write_netmsg(messages, &delta_decoders, &custom_messages);

                        data.msg = write.leak();
                    }
                    _ => (),
                }
            });
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
