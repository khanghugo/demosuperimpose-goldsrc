use hldemo::{Demo, FrameData};

use crate::{open_demo, writer::BitWriter};

use super::*;

// https://github.com/matthewearl/demsuperimpose/blob/master/demsuperimpose.py

// Turn each first person demo into a series of movement (take the origin and viewangle)
// Get spawnbaseline then assign the model for each demo (frame 0 3)
// Assign PacketEntity for entity initialization (frame 0 4)
// Assign the series of movements into delta packet entities. Be mindful of SvcTime for correct progression.

// Main demo and other demos must have same frametime.
// Make sure to make it offset-able.
struct GhostInfo {
    origin: Vec<[f32; 3]>,
    viewangles: Vec<[f32; 3]>,
    anim: Vec<Anim>,
}

struct Anim {
    sequence: Option<Vec<u8>>,
    frame: Option<Vec<u8>>,
    animtime: Option<Vec<u8>>,
}

pub fn superimpose<'a>(main: String, others: Vec<(String, f32)>) -> Demo<'a> {
    let (mut main_demo, other_demos) = parse_demos(main, &others);
    let ghost_info = get_ghost(other_demos);

    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    let mut main_demo_player_delta = Delta::new();
    // Model in entry 0 is different from entry 1.
    // Entry 0 is default model. Entry 1 is used model.
    let mut other_demos_indices: Vec<u16> = vec![];

    // Track the current ghost frame
    let mut current_frame_index = 0;

    // Work around the lack of understanding why frame value increases unreasonably.
    // f32 for alignment
    let mut other_demos_sequence_frame: Vec<f32> = vec![0.; ghost_info.len()];

    for (_, entry) in main_demo.directory.entries.iter_mut().enumerate() {
        for (_, frame) in entry.frames.iter_mut().enumerate() {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, mut messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                for message in &mut messages {
                    match message {
                        Message::EngineMessage(what) => match what {
                            EngineMessage::SvcSpawnBaseline(baseline) => {
                                for _ in 0..ghost_info.len() {
                                    // Find free entities indices.
                                    let mut current_free_entity = 0;
                                    let mut insert_idx = 0;

                                    for (idx, entity) in baseline.entities.iter().enumerate() {
                                        if entity.index.to_u16() == 1 {
                                            main_demo_player_delta = entity.delta.clone();
                                        }

                                        if entity.index.to_u16() == current_free_entity
                                            || other_demos_indices.contains(&current_free_entity)
                                        {
                                            current_free_entity += 1;
                                            insert_idx = idx + 1;
                                        } else {
                                            break;
                                        }
                                    }

                                    other_demos_indices.push(current_free_entity);

                                    // Insert new baseline.
                                    let mut other_demo_entity_idx = BitWriter::new();
                                    other_demo_entity_idx
                                        .append_u32_range(current_free_entity as u32, 11);

                                    let mut other_demo_type = BitWriter::new();
                                    other_demo_type.append_u32_range(1, 2);

                                    let mut other_demo_delta = Delta::new();

                                    // This modelindex is default model.
                                    other_demo_delta.insert(
                                        "modelindex\0".to_string(),
                                        main_demo_player_delta
                                            .get("modelindex\0")
                                            .clone()
                                            .unwrap()
                                            .to_vec(),
                                    );

                                    let mut other_demo_delta = main_demo_player_delta.clone();
                                    other_demo_delta.remove("gravity\0");
                                    other_demo_delta.remove("friction\0");
                                    other_demo_delta.remove("usehull\0");
                                    other_demo_delta.remove("spectator\0");

                                    baseline.entities.insert(
                                        insert_idx,
                                        EntityS {
                                            entity_index: other_demo_entity_idx.data.to_u16(),
                                            index: other_demo_entity_idx.data,
                                            type_: other_demo_type.data,
                                            delta: other_demo_delta,
                                        },
                                    );
                                }
                            }
                            EngineMessage::SvcPacketEntities(packet) => {
                                for what in &packet.entity_states {
                                    if what.entity_index == 1 {
                                        if what.delta.get("modelindex\0").is_some() {
                                            main_demo_player_delta = what.delta.clone();
                                        }
                                    }
                                }

                                for (ghost, ghost_entity_index) in
                                    ghost_info.iter().zip(&other_demos_indices)
                                {
                                    let mut new_entity_count = BitWriter::new();
                                    new_entity_count
                                        .append_u32_range(packet.entity_count.to_u32() + 1, 16);

                                    // Change count.
                                    packet.entity_count = new_entity_count.data;

                                    if ghost.origin.len() <= current_frame_index {
                                        continue;
                                    }

                                    let mut other_demo_entity_state_delta = Delta::new();

                                    other_demo_entity_state_delta.insert(
                                        "origin[0]\0".to_string(),
                                        ghost.origin[current_frame_index][0].to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[1]\0".to_string(),
                                        ghost.origin[current_frame_index][1].to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[2]\0".to_string(),
                                        ghost.origin[current_frame_index][2].to_le_bytes().to_vec(),
                                    );

                                    other_demo_entity_state_delta.insert(
                                        "modelindex\0".to_string(),
                                        main_demo_player_delta
                                            .get("modelindex\0")
                                            .unwrap()
                                            .to_owned(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "framerate\0".to_string(),
                                        0.01f32.to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "controller[0]\0".to_string(),
                                        127u32.to_le_bytes().to_vec(),
                                    );

                                    let mut other_demo_absolute_entity_index = BitWriter::new();
                                    other_demo_absolute_entity_index
                                        .append_u32_range(*ghost_entity_index as u32, 11);

                                    // Insert entity then change the value for entity index difference correctly.
                                    let mut insert_index = 0;
                                    for entity in &packet.entity_states {
                                        if entity.entity_index > *ghost_entity_index {
                                            break;
                                        }

                                        insert_index += 1;
                                    }

                                    // Entity 0 is always there so there is no need to handle weird case where ghost index is 0.
                                    // Insert between insert entity and ghost entity
                                    let before_entity = &packet.entity_states[insert_index - 1];
                                    let mut ghost_entity_index_difference = BitWriter::new();

                                    ghost_entity_index_difference.append_u32_range(
                                        (ghost_entity_index - before_entity.entity_index) as u32,
                                        6,
                                    );

                                    let other_demo_entity_state = EntityState {
                                        entity_index: *ghost_entity_index, // This doesn't really do anything but for you to read.
                                        increment_entity_number: false,
                                        is_absolute_entity_index: Some(false),
                                        absolute_entity_index: None,
                                        entity_index_difference: Some(
                                            ghost_entity_index_difference.data,
                                        ),
                                        has_custom_delta: false,
                                        has_baseline_index: false,
                                        baseline_index: None,
                                        delta: other_demo_entity_state_delta,
                                    };

                                    // Insert between ghost entity and next entity.
                                    // If it is last entity then there is no need to change.
                                    if insert_index != packet.entity_states.len() - 1 {
                                        let next_entity = &mut packet.entity_states[insert_index];
                                        let difference =
                                            next_entity.entity_index - ghost_entity_index;

                                        let mut next_entity_index_difference = BitWriter::new();
                                        next_entity_index_difference
                                            .append_u32_range(difference as u32, 6);

                                        next_entity.entity_index_difference =
                                            Some(next_entity_index_difference.data);
                                    }

                                    packet
                                        .entity_states
                                        .insert(insert_index, other_demo_entity_state);
                                }
                            }
                            EngineMessage::SvcDeltaPacketEntities(packet) => {
                                for (enumerate_index, (ghost, ghost_entity_index)) in
                                    ghost_info.iter().zip(&other_demos_indices).enumerate()
                                {
                                    let mut new_entity_count = BitWriter::new();
                                    new_entity_count
                                        .append_u32_range(packet.entity_count.to_u32() + 1, 16);

                                    packet.entity_count = new_entity_count.data;

                                    if ghost.origin.len() <= current_frame_index {
                                        continue;
                                    }

                                    // Append at the end to avoid arithmetic.
                                    let mut other_demo_entity_state_delta = Delta::new();

                                    other_demo_entity_state_delta.insert(
                                        "origin[0]\0".to_string(),
                                        ghost.origin[current_frame_index][0].to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[1]\0".to_string(),
                                        ghost.origin[current_frame_index][1].to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[2]\0".to_string(),
                                        ghost.origin[current_frame_index][2].to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[0]\0".to_string(),
                                        (ghost.viewangles[current_frame_index][0] * -1.)
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[1]\0".to_string(),
                                        ghost.viewangles[current_frame_index][1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[2]\0".to_string(),
                                        ghost.viewangles[current_frame_index][2]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    if let Some(sequence) =
                                        &ghost.anim[current_frame_index].sequence
                                    {
                                        other_demo_entity_state_delta
                                            .insert("sequence\0".to_string(), sequence.to_vec());
                                        other_demos_sequence_frame[enumerate_index] = 0.;
                                    }
                                    if let Some(_) = &ghost.anim[current_frame_index].frame {
                                        other_demo_entity_state_delta.insert(
                                            "frame\0".to_string(),
                                            other_demos_sequence_frame[enumerate_index]
                                                .to_le_bytes()
                                                .to_vec(),
                                        );
                                        other_demos_sequence_frame[enumerate_index] += 1.;
                                    }
                                    if let Some(animtime) =
                                        &ghost.anim[current_frame_index].animtime
                                    {
                                        other_demo_entity_state_delta
                                            .insert("animtime\0".to_string(), animtime.to_vec());
                                    }

                                    let mut other_demo_absolute_entity_index = BitWriter::new();
                                    other_demo_absolute_entity_index
                                        .append_u32_range(*ghost_entity_index as u32, 11);

                                    // Insert entity then change the value for entity index difference correctly.
                                    let mut insert_index = 0;
                                    for entity in &packet.entity_states {
                                        if entity.entity_index > *ghost_entity_index {
                                            break;
                                        }

                                        insert_index += 1;
                                    }

                                    // Entity 0 is always there so there is no need to handle weird case where ghost index is 0.
                                    // Insert between insert entity and ghost entity
                                    let before_entity = &packet.entity_states[insert_index - 1];
                                    let mut ghost_entity_index_difference = BitWriter::new();

                                    ghost_entity_index_difference.append_u32_range(
                                        (ghost_entity_index - before_entity.entity_index) as u32,
                                        6,
                                    );

                                    let other_demo_entity_state = EntityStateDelta {
                                        entity_index: *ghost_entity_index, // This doesn't really do anything but for you to read.
                                        remove_entity: false,
                                        is_absolute_entity_index: false,
                                        absolute_entity_index: None,
                                        entity_index_difference: Some(
                                            ghost_entity_index_difference.data,
                                        ),
                                        has_custom_delta: Some(false),
                                        delta: Some(other_demo_entity_state_delta),
                                    };

                                    // Insert between ghost entity and next entity.
                                    // If it is last entity then there is no need to change.
                                    if insert_index < packet.entity_states.len() {
                                        let next_entity = &mut packet.entity_states[insert_index];
                                        let difference =
                                            next_entity.entity_index - ghost_entity_index;

                                        let mut next_entity_index_difference = BitWriter::new();
                                        next_entity_index_difference
                                            .append_u32_range(difference as u32, 6);

                                        next_entity.entity_index_difference =
                                            Some(next_entity_index_difference.data);
                                    }

                                    packet
                                        .entity_states
                                        .insert(insert_index, other_demo_entity_state);
                                }

                                // Only increment after we find our frame.
                                current_frame_index += 1;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }

                let write = write_netmsg(messages, &delta_decoders, &custom_messages);
                data.msg = write.leak();
            }
        }
    }

    main_demo
}

fn parse_demos<'a>(main_demo: String, others: &Vec<(String, f32)>) -> (Demo<'a>, Vec<Demo<'a>>) {
    let main_demo = open_demo!(main_demo);
    let mut other_demos: Vec<Demo> = vec![];

    for (other, _) in others {
        let other_demo = open_demo!(other);
        other_demos.push(other_demo);
    }

    (main_demo, other_demos)
}

fn get_ghost<'a>(other_demos: Vec<Demo<'a>>) -> Vec<GhostInfo> {
    other_demos
        .iter()
        .map(|other_demo| {
            let mut origin: Vec<[f32; 3]> = vec![];
            let mut viewangles: Vec<[f32; 3]> = vec![];
            let mut anim: Vec<Anim> = vec![];

            let mut delta_decoders = get_initial_delta();
            let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();
            for (_, entry) in other_demo.directory.entries.iter().enumerate() {
                for frame in &entry.frames {
                    match &frame.data {
                        FrameData::NetMsg((_, data)) => {
                            let (_, messages) =
                                parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages)
                                    .unwrap();

                            for message in messages {
                                match message {
                                    Message::EngineMessage(what) => match what {
                                        EngineMessage::SvcDeltaPacketEntities(what) => {
                                            let mut sequence: Option<Vec<u8>> = None;
                                            let mut frame: Option<Vec<u8>> = None;
                                            let mut animtime: Option<Vec<u8>> = None;

                                            for entity in &what.entity_states {
                                                if entity.entity_index == 1
                                                    && entity.delta.is_some()
                                                {
                                                    sequence = entity
                                                        .delta
                                                        .as_ref()
                                                        .unwrap()
                                                        .get("gaitsequence\0")
                                                        .cloned();
                                                    frame = entity
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
                                            // origin.push(data.info.ref_params.vieworg);
                                            // viewangles.push(data.info.ref_params.viewangles);
                                            anim.push(Anim {
                                                sequence,
                                                frame,
                                                animtime,
                                            });
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                        }
                        FrameData::ClientData(what) => {
                            origin.push(what.origin);
                            viewangles.push(what.viewangles);
                        }
                        _ => (),
                    }
                }
            }

            // Final offset to anim vector to have equal length with the other two.
            for i in 0..(origin.len() - anim.len()) {
                anim.push(Anim {
                    sequence: None,
                    frame: None,
                    animtime: None,
                });
            }

            GhostInfo {
                origin,
                viewangles,
                anim,
            }
        })
        .collect()
}
