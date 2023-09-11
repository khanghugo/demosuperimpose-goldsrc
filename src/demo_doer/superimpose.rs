use hldemo::{Demo, FrameData};
use nom::{
    bytes::complete::{take, take_till, take_until},
    character::{complete::newline, is_newline},
    combinator::map,
    sequence::{self, tuple},
    IResult,
};

use crate::{open_demo, writer::BitWriter};

use super::*;

struct GhostFrame {
    origin: [f32; 3],
    viewangles: [f32; 3],
    sequence: Option<Vec<u8>>,
    frame: Option<Vec<u8>>,
    animtime: Option<Vec<u8>>,
}

impl GhostFrame {
    pub fn get_origin(&self) -> [f32; 3] {
        self.origin
    }

    pub fn get_viewangles(&self) -> [f32; 3] {
        self.viewangles
    }

    pub fn get_sequence(&self) -> Option<&Vec<u8>> {
        self.sequence.as_ref()
    }

    pub fn get_anim_frame(&self) -> Option<&Vec<u8>> {
        self.frame.as_ref()
    }

    pub fn get_animtime(&self) -> Option<&Vec<u8>> {
        self.animtime.as_ref()
    }
}

struct GhostInfo {
    ghost_name: String,
    entity_index: u16,
    frames: Vec<GhostFrame>,
    ghost_anim_frame: f32,
}

impl GhostInfo {
    pub fn new() -> Self {
        Self {
            ghost_name: "".to_string(),
            entity_index: 0,
            frames: vec![],
            ghost_anim_frame: 0.,
        }
    }

    pub fn append_frame(
        &mut self,
        origin: [f32; 3],
        viewangles: [f32; 3],
        sequence: Option<Vec<u8>>,
        frame: Option<Vec<u8>>,
        animtime: Option<Vec<u8>>,
    ) {
        self.frames.push(GhostFrame {
            origin,
            viewangles,
            sequence,
            frame,
            animtime,
        });
    }

    pub fn get_frame(&self, idx: usize) -> &GhostFrame {
        // Eh
        self.frames.get(idx).unwrap()
    }

    pub fn get_size(&self) -> usize {
        self.frames.len()
    }

    pub fn set_name(&mut self, name: String) {
        self.ghost_name = name.to_owned();
    }

    pub fn get_name(&self) -> String {
        self.ghost_name.to_owned()
    }

    pub fn set_entity_index(&mut self, idx: u16) {
        self.entity_index = idx;
    }

    pub fn get_entity_index(&self) -> u16 {
        self.entity_index
    }

    pub fn increment_ghost_anim_frame(&mut self) {
        self.ghost_anim_frame += 1.;
    }

    pub fn reset_ghost_anim_frame(&mut self) {
        self.ghost_anim_frame = 0.;
    }

    pub fn get_ghost_anim_frame(&self) -> f32 {
        self.ghost_anim_frame
    }
}

pub fn superimpose<'a>(main: String, others: Vec<(String, f32)>) -> Demo<'a> {
    println!("Total demos: {} + 1", others.len());

    let (mut main_demo, other_demos) = parse_demos(main, &others);
    let mut ghosts =
        get_ghost_from_demos(other_demos, others.iter().map(|e| e.0.to_owned()).collect());

    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    let mut main_demo_player_delta = Delta::new();

    // This keeps track of the currently available entity_index for ghost.
    let mut other_demos_indices: Vec<u16> = vec![];

    // Track the current ghost frame
    let mut current_frame_index = 0;

    // Work around the lack of understanding why frame value increases unreasonably.
    // f32 for alignment
    let mut other_demos_sequence_frame: Vec<f32> = vec![0.; ghosts.len()];

    for (_, entry) in main_demo.directory.entries.iter_mut().enumerate() {
        for (_, frame) in entry.frames.iter_mut().enumerate() {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, mut messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                for message in &mut messages {
                    match message {
                        Message::EngineMessage(what) => match what {
                            EngineMessage::SvcSpawnBaseline(baseline) => {
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
                                    let mut other_demo_entity_idx = BitWriter::new();
                                    other_demo_entity_idx
                                        .append_u32_range(current_free_entity as u32, 11);

                                    let mut other_demo_type = BitWriter::new();
                                    other_demo_type.append_u32_range(1, 2);

                                    // let mut other_demo_delta = Delta::new();
                                    // This modelindex is default model.
                                    // other_demo_delta.insert(
                                    //     "modelindex\0".to_string(),
                                    //     main_demo_player_delta
                                    //         .get("modelindex\0")
                                    //         .clone()
                                    //         .unwrap()
                                    //         .to_vec(),
                                    // );

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

                                for ghost in ghosts.iter() {
                                    let mut new_entity_count = BitWriter::new();
                                    new_entity_count
                                        .append_u32_range(packet.entity_count.to_u32() + 1, 16);

                                    // Change count.
                                    packet.entity_count = new_entity_count.data;

                                    if ghost.get_size() <= current_frame_index {
                                        continue;
                                    }

                                    let mut other_demo_entity_state_delta = Delta::new();

                                    other_demo_entity_state_delta.insert(
                                        "origin[0]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_origin()[0]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[1]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_origin()[1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[2]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_origin()[2]
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
                                    other_demo_entity_state_delta.insert(
                                        "framerate\0".to_string(),
                                        0.01f32.to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "controller[0]\0".to_string(),
                                        127u32.to_le_bytes().to_vec(),
                                    );
                                    other_demo_entity_state_delta
                                        .insert("solid\0".to_string(), 4u32.to_le_bytes().to_vec());
                                    other_demo_entity_state_delta.insert(
                                        "movetype\0".to_string(),
                                        7u32.to_le_bytes().to_vec(),
                                    );

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
                                    let mut ghost_absolute_entity_index: Option<BitType> = None;
                                    let mut ghost_entity_index_difference: Option<BitType> = None;

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
                            }
                            EngineMessage::SvcDeltaPacketEntities(packet) => {
                                for ghost in ghosts.iter_mut() {
                                    let mut new_entity_count = BitWriter::new();
                                    new_entity_count
                                        .append_u32_range(packet.entity_count.to_u32() + 1, 16);
                                    packet.entity_count = new_entity_count.data;

                                    if ghost.get_size() <= current_frame_index {
                                        continue;
                                    }

                                    let mut other_demo_entity_state_delta = Delta::new();

                                    // Origin/viewangles
                                    other_demo_entity_state_delta.insert(
                                        "origin[0]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_origin()[0]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[1]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_origin()[1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "origin[2]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_origin()[2]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[0]\0".to_string(),
                                        (ghost.get_frame(current_frame_index).get_viewangles()[0]
                                            * -1.)
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[1]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_viewangles()[1]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );
                                    other_demo_entity_state_delta.insert(
                                        "angles[2]\0".to_string(),
                                        ghost.get_frame(current_frame_index).get_viewangles()[2]
                                            .to_le_bytes()
                                            .to_vec(),
                                    );

                                    // Animation
                                    // Eh, I dont know.
                                    if let Some(sequence) =
                                        ghost.get_frame(current_frame_index).get_sequence()
                                    {
                                        other_demo_entity_state_delta
                                            .insert("sequence\0".to_string(), sequence.to_vec());
                                        ghost.reset_ghost_anim_frame();
                                    }

                                    if let Some(_) =
                                        ghost.get_frame(current_frame_index).get_anim_frame()
                                    {
                                        // It uses tracked value for frame value.
                                        other_demo_entity_state_delta.insert(
                                            "frame\0".to_string(),
                                            ghost.get_ghost_anim_frame().to_le_bytes().to_vec(),
                                        );
                                        ghost.increment_ghost_anim_frame();
                                    }

                                    if let Some(animtime) =
                                        ghost.get_frame(current_frame_index).get_animtime()
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

                                    // Entity 0 is always there so there is no need to handle weird case where ghost index is 0.
                                    // Insert between insert entity and ghost entity
                                    let before_entity = &packet.entity_states[insert_index - 1];
                                    let mut is_absolute_entity_index = false;
                                    let mut ghost_absolute_entity_index: Option<BitType> = None;
                                    let mut ghost_entity_index_difference: Option<BitType> = None;

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

                                    let other_demo_entity_state = EntityStateDelta {
                                        entity_index: ghost.get_entity_index(), // This doesn't really do anything but for you to read.
                                        remove_entity: false,
                                        is_absolute_entity_index,
                                        absolute_entity_index: ghost_absolute_entity_index,
                                        entity_index_difference: ghost_entity_index_difference,
                                        has_custom_delta: Some(false),
                                        delta: Some(other_demo_entity_state_delta),
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

                                // Only increment after we add entity update.
                                current_frame_index += 1;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }

                let write = write_netmsg(messages, &mut delta_decoders, &custom_messages);
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

fn get_ghost_from_demos<'a>(
    other_demos: Vec<Demo<'a>>,
    other_demos_names: Vec<String>,
) -> Vec<GhostInfo> {
    other_demos
        .iter()
        .enumerate()
        .map(|(demo_idx, other_demo)| {
            // New ghost
            let mut ghost = GhostInfo::new();
            ghost.set_name(other_demos_names[demo_idx].to_owned());
            ghost.reset_ghost_anim_frame();

            let mut delta_decoders = get_initial_delta();
            let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

            // Help with checking out which demo is unparse-able.
            println!("Last parsed demo {}", other_demos_names[demo_idx]);

            // Because player origin/viewangles and animation are on different frame, we have to sync it.
            // Order goes: players info > animation > player info > ...
            let mut sequence: Option<Vec<u8>> = None;
            let mut anim_frame: Option<Vec<u8>> = None;
            let mut animtime: Option<Vec<u8>> = None;

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
        })
        .collect()
}

// fn get_ghost_from_simen_wrbots<'a>(wrbots: Vec<&'a [u8]>) -> Vec<GhostInfo> {}

fn skip_line(i: &str) -> IResult<&str, u8> {
    map(tuple((take_till(|c| c == '\n'), take(1usize))), |what| 0u8)(i)
}

fn simen_wrbot_header(i: &str) -> IResult<&str, u8> {
    map(
        tuple((
            skip_line, // Time
            skip_line, // Name
            skip_line, // SteamID
            skip_line, // Date
            skip_line, // Location
            skip_line, // ??
        )),
        |what| 0u8,
    )(i)
}

// fn simen_wrbot_line(i: &str) -> IResult<&str> {}
