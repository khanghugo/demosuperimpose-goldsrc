use dem::{
    hldemo::{Demo, FrameData},
    parse_netmsg,
    types::{EngineMessage, NetMessage},
    Aux,
};

use super::*;

pub fn demo_ghost_parse<'a>(
    name: &str,
    demo: &Demo<'a>,
    offset: f32,
    parse_anim: bool,
) -> GhostInfo {
    // New ghost
    let mut ghost = GhostInfo::new();
    ghost.set_name(name.to_owned());
    ghost.reset_ghost_anim_frame();

    let mut aux = Aux::new();

    // Help with checking out which demo is unparse-able.
    // println!("Last parsed demo {}", ghost.get_name());

    // Because player origin/viewangles and animation are on different frame, we have to sync it.
    // Order goes: players info > animation > player info > ...
    let mut sequence: Option<Vec<u8>> = None;
    let mut anim_frame: Option<Vec<u8>> = None;
    let mut animtime: Option<Vec<u8>> = None;

    for (_, entry) in demo.directory.entries.iter().enumerate() {
        for frame in &entry.frames {
            match &frame.data {
                FrameData::NetMsg((_, data)) => {
                    if !parse_anim {
                        continue;
                    }

                    let (_, messages) = parse_netmsg(data.msg, &mut aux).unwrap();

                    for message in messages {
                        match message {
                            NetMessage::EngineMessage(what) => match *what {
                                EngineMessage::SvcDeltaPacketEntities(what) => {
                                    for entity in &what.entity_states {
                                        if entity.entity_index == 1 && entity.delta.is_some() {
                                            sequence = entity
                                                .delta
                                                .as_ref()
                                                .unwrap()
                                                .get("gaitsequence\0")
                                                .cloned();
                                            anim_frame = entity
                                                .delta
                                                .as_ref()
                                                .unwrap()
                                                .get("frame\0")
                                                .cloned();
                                            animtime = entity
                                                .delta
                                                .as_ref()
                                                .unwrap()
                                                .get("animtime\0")
                                                .cloned();
                                        }
                                    }
                                    // These numbers are not very close to what we want.
                                    // They are vieworigin, not player origin.
                                    // origin.push(data.info.ref_params.vieworg);
                                    // viewangles.push(data.info.ref_params.viewangles);
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                }
                FrameData::ClientData(what) => {
                    // Append frame on this frame because the demo orders like it.
                    ghost.append_frame(
                        what.origin,
                        what.viewangles,
                        sequence.to_owned(),
                        anim_frame.to_owned(),
                        animtime.to_owned(),
                        None,
                    );

                    // Reset for next find.
                    sequence = None;
                    anim_frame = None;
                    animtime = None;
                }
                _ => (),
            }
        }
    }

    ghost
}
