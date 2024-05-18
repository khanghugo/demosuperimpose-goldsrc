use std::{fs, io::Write, path::PathBuf};

use dem::{
    bit::{BitSliceCast, BitWriter}, nbit_num, parse_netmsg, types::{BitVec, Delta, EngineMessage, EntityS, EntityState, EntityStateDelta, NetMessage}, write_netmsg, Aux
};

use crate::{demo_doer::superimpose::get_ghost::get_ghosts, open_demo};

use super::*;

pub fn superimpose_folder<'a>(main: String, folder: String) -> Demo<'a> {
    let pathbuf = PathBuf::from(folder);
    let files = fs::read_dir(pathbuf).unwrap();

    let others: Vec<(String, f32)> = files
        .map(|file| (file.unwrap().path().to_str().unwrap().to_owned(), 0.))
        .collect();

    superimpose(main, others)
}

pub fn superimpose<'a>(main: String, others: Vec<(String, f32)>) -> Demo<'a> {
    println!("Total demos: {} + 1", others.len());

    let mut main_demo = open_demo!(main);
    let mut ghosts = get_ghosts(&others);
    // New line for our print finally
    println!("");

    let mut aux = Aux::new();

    let mut main_demo_player_delta = Delta::new();

    // This keeps track of the currently available entity_index for ghost.
    let mut other_demos_indices: Vec<u16> = vec![];

    // Track the current ghost frame
    let mut current_frame_index = 0;

    for (entry_idx, entry) in main_demo.directory.entries.iter_mut().enumerate() {
        let frame_count = entry.frames.len();
        for (frame_idx, frame) in entry.frames.iter_mut().enumerate() {
            print!(
                "\rWorking on entry {} frame {} out of {}   ",
                entry_idx, frame_idx, frame_count
            );
            std::io::stdout().flush().unwrap();

            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, mut messages) = parse_netmsg(data.msg, &mut aux).unwrap();

                for message in &mut messages {
                    match message {
                        NetMessage::EngineMessage(what) => match **what {
                            EngineMessage::SvcSpawnBaseline(ref mut baseline) => {
                                for ghost in &mut ghosts {
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
                                    ghost.set_entity_index(current_free_entity);

                                    // Insert new baseline.
                                    let other_demo_entity_idx = nbit_num!(current_free_entity, 11);
                                    let other_demo_type = nbit_num!(1, 2);

                                    let mut other_demo_delta = main_demo_player_delta.clone();
                                    other_demo_delta.remove("gravity\0");
                                    other_demo_delta.remove("friction\0");
                                    other_demo_delta.remove("usehull\0");
                                    other_demo_delta.remove("spectator\0");

                                    // This modelindex is default model.
                                    other_demo_delta.insert(
                                        "modelindex\0".to_string(),
                                        main_demo_player_delta
                                            .get("modelindex\0")
                                            .clone()
                                            .unwrap()
                                            .to_vec(),
                                    );

                                    baseline.entities.insert(
                                        insert_idx,
                                        EntityS {
                                            entity_index: other_demo_entity_idx.to_u16(),
                                            index: other_demo_entity_idx,
                                            type_: other_demo_type,
                                            delta: other_demo_delta,
                                        },
                                    );
                                }
                            }
                            EngineMessage::SvcPacketEntities(ref mut packet) => {
                                for what in &packet.entity_states {
                                    if what.entity_index == 1 {
                                        if what.delta.get("modelindex\0").is_some() {
                                            main_demo_player_delta = what.delta.clone();
                                        }
                                    }
                                }

                                for ghost in ghosts.iter() {
                                    // Change count.
                                    packet.entity_count =
                                        nbit_num!(packet.entity_count.to_u32() + 1, 16);

                                    if ghost.get_size() <= current_frame_index {
                                        continue;
                                    }

                                    let mut other_demo_entity_state_delta = Delta::new();

                                    other_demo_entity_state_delta.insert(
                                        "origin[0]\0".to_string(),
                                        ghost.get_frame(current_frame_index).origin[0]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[1]\0".to_string(),
                                        ghost.get_frame(current_frame_index).origin[1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[2]\0".to_string(),
                                        ghost.get_frame(current_frame_index).origin[2]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );

                                    other_demo_entity_state_delta.insert(
                                        "modelindex\0".to_string(),
                                        main_demo_player_delta
                                            .get("modelindex\0")
                                            .unwrap()
                                            .to_owned(),
                                    );
                                    // other_demo_entity_state_delta.insert(
                                    //     "framerate\0".to_string(),
                                    //     0.01f32.to_le_bytes().to_vec(),
                                    // );
                                    // other_demo_entity_state_delta.insert(
                                    //     "controller[0]\0".to_string(),
                                    //     127u32.to_le_bytes().to_vec(),
                                    // );
                                    // other_demo_entity_state_delta
                                    //     .insert("solid\0".to_string(), 4u32.to_le_bytes().to_vec());
                                    // other_demo_entity_state_delta.insert(
                                    //     "movetype\0".to_string(),
                                    //     7u32.to_le_bytes().to_vec(),
                                    // );

                                    // Insert entity then change the value for entity index difference correctly.
                                    let mut insert_index = 0;
                                    for entity in &packet.entity_states {
                                        if entity.entity_index > ghost.get_entity_index() {
                                            break;
                                        }

                                        insert_index += 1;
                                    }

                                    // Entity 0 is always there so there is no need to handle weird case where ghost index is 0.
                                    // Insert between insert entity and ghost entity
                                    let before_entity = &packet.entity_states[insert_index - 1];
                                    let mut is_absolute_entity_index = false;
                                    let mut ghost_absolute_entity_index: Option<BitVec> = None;
                                    let mut ghost_entity_index_difference: Option<BitVec> = None;

                                    // If difference is more than 63, we do absolute entity index instead.
                                    // The reason is that difference is only 6 bits, so 63 max.
                                    let difference =
                                        ghost.get_entity_index() - before_entity.entity_index;
                                    if difference > (1 << 6) - 1 {
                                        let mut index = BitWriter::new();
                                        index.append_u32_range(ghost.get_entity_index() as u32, 11);

                                        ghost_absolute_entity_index = Some(index.data.to_owned());
                                        is_absolute_entity_index = true;
                                    } else {
                                        let mut diff = BitWriter::new();
                                        diff.append_u32_range(
                                            (ghost.get_entity_index() - before_entity.entity_index)
                                                as u32,
                                            6,
                                        );
                                        ghost_entity_index_difference = Some(diff.data.to_owned());
                                    }

                                    let other_demo_entity_state = EntityState {
                                        entity_index: ghost.get_entity_index(), // This doesn't really do anything but for you to read.
                                        increment_entity_number: false,
                                        is_absolute_entity_index: Some(is_absolute_entity_index),
                                        absolute_entity_index: ghost_absolute_entity_index,
                                        entity_index_difference: ghost_entity_index_difference,
                                        has_custom_delta: false,
                                        has_baseline_index: false,
                                        baseline_index: None,
                                        delta: other_demo_entity_state_delta,
                                    };

                                    // Insert between ghost entity and next entity.
                                    // If it is last entity then there is no need to change.
                                    if insert_index < packet.entity_states.len() {
                                        let next_entity = &mut packet.entity_states[insert_index];
                                        let difference =
                                            next_entity.entity_index - ghost.get_entity_index();

                                        if difference > (1 << 6) - 1 {
                                            // It is possible that by the time this is hit,
                                            // the next entity is already numbered by absolute index.
                                        } else {
                                            let mut next_entity_index_difference = BitWriter::new();
                                            next_entity_index_difference
                                                .append_u32_range(difference as u32, 6);

                                            next_entity.entity_index_difference =
                                                Some(next_entity_index_difference.data);
                                        }
                                    }

                                    packet
                                        .entity_states
                                        .insert(insert_index, other_demo_entity_state);
                                }

                                if packet.entity_count.to_u16() >= 256 {
                                    println!("");
                                    panic!("Exceeding 256 entities update limit ({} entities). Demo will not work.", packet.entity_count.to_u16())
                                }
                            }
                            EngineMessage::SvcDeltaPacketEntities(ref mut packet) => {
                                for ghost in ghosts.iter_mut() {
                                    // Increment entity count because we have ghost
                                    // Should increase before the continue line because we don't remove entity.
                                    packet.entity_count =
                                        nbit_num!(packet.entity_count.to_u32() + 1, 16);

                                    if ghost.get_size() <= current_frame_index {
                                        continue;
                                    }

                                    let mut other_demo_entity_state_delta = Delta::new();

                                    // Origin/viewangles
                                    other_demo_entity_state_delta.insert(
                                        "origin[0]\0".to_string(),
                                        ghost.get_frame(current_frame_index).origin[0]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[1]\0".to_string(),
                                        ghost.get_frame(current_frame_index).origin[1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[2]\0".to_string(),
                                        ghost.get_frame(current_frame_index).origin[2]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[0]\0".to_string(),
                                        (ghost.get_frame(current_frame_index).viewangles[0] * -1.)
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[1]\0".to_string(),
                                        ghost.get_frame(current_frame_index).viewangles[1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[2]\0".to_string(),
                                        ghost.get_frame(current_frame_index).viewangles[2]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );

                                    // Animation
                                    // Eh, I dont know.
                                    if let Some(sequence) =
                                        &ghost.get_frame(current_frame_index).sequence
                                    {
                                        other_demo_entity_state_delta
                                            .insert("sequence\0".to_string(), sequence.to_vec());
                                        ghost.reset_ghost_anim_frame();
                                    }

                                    if let Some(_) = ghost.get_frame(current_frame_index).frame {
                                        // It uses tracked value for frame value.
                                        other_demo_entity_state_delta.insert(
                                            "frame\0".to_string(),
                                            ghost.get_ghost_anim_frame().to_le_bytes().to_vec(),
                                        );
                                        ghost.increment_ghost_anim_frame();
                                    }

                                    if let Some(animtime) =
                                        &ghost.get_frame(current_frame_index).animtime
                                    {
                                        other_demo_entity_state_delta
                                            .insert("animtime\0".to_string(), animtime.to_vec());
                                    }

                                    // Insert entity then change the value for entity index difference correctly.
                                    let mut insert_index = 0;
                                    for entity in &packet.entity_states {
                                        if entity.entity_index > ghost.get_entity_index() {
                                            break;
                                        }

                                        insert_index += 1;
                                    }

                                    // If there is no update, subscribing to `entity_states` array would be out of index.
                                    let other_demo_entity_state = if insert_index > 0 {
                                        // Insert between insert entity and ghost entity
                                        let before_entity = &packet.entity_states[insert_index - 1];
                                        let mut is_absolute_entity_index = false;
                                        let mut ghost_absolute_entity_index: Option<BitVec> = None;
                                        let mut ghost_entity_index_difference: Option<BitVec> =
                                            None;

                                        // If difference is more than 63, we do absolute entity index instead.
                                        // The reason is that difference is only 6 bits, so 63 max.
                                        let difference =
                                            ghost.get_entity_index() - before_entity.entity_index;
                                        if difference > (1 << 6) - 1 {
                                            ghost_absolute_entity_index =
                                                Some(nbit_num!(ghost.get_entity_index(), 11));
                                            is_absolute_entity_index = true;
                                        } else {
                                            ghost_entity_index_difference = Some(nbit_num!(
                                                ghost.get_entity_index()
                                                    - before_entity.entity_index,
                                                6
                                            ));
                                        }

                                        EntityStateDelta {
                                            entity_index: ghost.get_entity_index(), // This doesn't really do anything but for you to read.
                                            remove_entity: false,
                                            is_absolute_entity_index,
                                            absolute_entity_index: ghost_absolute_entity_index,
                                            entity_index_difference: ghost_entity_index_difference,
                                            has_custom_delta: Some(false),
                                            delta: Some(other_demo_entity_state_delta),
                                        }
                                    } else {
                                        EntityStateDelta {
                                            entity_index: ghost.get_entity_index(), // This doesn't really do anything but for you to read.
                                            remove_entity: false,
                                            is_absolute_entity_index: true,
                                            absolute_entity_index: Some(nbit_num!(
                                                ghost.get_entity_index(),
                                                11
                                            )),
                                            entity_index_difference: None,
                                            has_custom_delta: Some(false),
                                            delta: Some(other_demo_entity_state_delta),
                                        }
                                    };

                                    // Insert between ghost entity and next entity.
                                    // If it is last entity then there is no need to change.
                                    if insert_index < packet.entity_states.len() {
                                        let next_entity = &mut packet.entity_states[insert_index];
                                        let difference =
                                            next_entity.entity_index - ghost.get_entity_index();

                                        if difference > (1 << 6) - 1 {
                                            // It is possible that by the time this is hit,
                                            // the next entity is already numbered by absolute index.
                                        } else {
                                            next_entity.entity_index_difference =
                                                Some(nbit_num!(difference, 6));
                                        }
                                    }

                                    packet
                                        .entity_states
                                        .insert(insert_index, other_demo_entity_state);
                                }

                                // Only increment after we add entity update.
                                current_frame_index += 1;

                                if packet.entity_count.to_u16() >= 256 {
                                    println!("");
                                    panic!("Exceeding 256 entities update limit ({} entities). Demo will not work.", packet.entity_count.to_u16())
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }

                let write = write_netmsg(messages, aux.clone());
                data.msg = write.leak();
            }
        }
    }

    // Newline print.
    println!("");

    main_demo
}
