// bsp info to insert
// ghost demo to insert

/// frame 0: SvcServerInfo SvcDeltaDescription SvcSetView SvcNewMovevars
///
/// SvcSetView needs setting to 1 otherwise game crash
///
/// SvcNewMovevars is needed otherwise game black screen
///
/// SvcServerInfo: wrong checksum ok
///
/// frame 1: SvcResourceList
/// frame 2: 8 nopes, omittable
/// frame 3: SvcSpawnBaseline, SvcSignOnNum(_) = 1,
/// frame 4: svcpackent entity
///
/// Total needed frames is 4
use bitvec::bitvec;
use bitvec::prelude::*;

use demosuperimpose_goldsrc::get_cs_delta_msg;
use demosuperimpose_goldsrc::netmsg_doer::delta_description::DeltaDescription;
use demosuperimpose_goldsrc::netmsg_doer::new_movevars::NewMovevars;
use demosuperimpose_goldsrc::netmsg_doer::server_info::ServerInfo;
use demosuperimpose_goldsrc::netmsg_doer::set_view::SetView;
use demosuperimpose_goldsrc::netmsg_doer::sign_on_num::SignOnNum;
use demosuperimpose_goldsrc::netmsg_doer::sound::Sound;
use demosuperimpose_goldsrc::netmsg_doer::spawn_baseline::SpawnBaseline;
use demosuperimpose_goldsrc::netmsg_doer::NetMsgDoerWithDelta;
use demosuperimpose_goldsrc::netmsg_doer::NetMsgDoerWithExtraInfo;
use demosuperimpose_goldsrc::rand_int_range;
use demosuperimpose_goldsrc::types::Delta;
use demosuperimpose_goldsrc::types::DeltaDecoderTable;
use demosuperimpose_goldsrc::types::EntityS;
use demosuperimpose_goldsrc::types::OriginCoord;
use demosuperimpose_goldsrc::types::SvcNewMoveVars;
use demosuperimpose_goldsrc::types::SvcServerInfo;
use demosuperimpose_goldsrc::types::SvcSetView;
use demosuperimpose_goldsrc::types::SvcSignOnNum;
use demosuperimpose_goldsrc::types::SvcSpawnBaseline;
use demosuperimpose_goldsrc::utils::Buttons;
use demosuperimpose_goldsrc::wrap_message;
use demosuperimpose_goldsrc::{
    init_parse, nbit_num, nbit_str,
    netmsg_doer::{
        parse_netmsg, parse_netmsg_immutable, resource_list::ResourceList, write_netmsg, NetMsgDoer,
    },
    types::Resource,
    types::{EngineMessage, Message, SvcDeltaDescription, SvcResourceList, SvcSound},
    utils::{NetMsgDataMethods, ResourceType},
};
use hldemo::parse::frame::netmsg;
use hldemo::{
    parse::frame, ClientDataData, Demo, DemoBufferData, Frame, FrameData, MoveVars, NetMsgData,
    NetMsgFrameType, NetMsgInfo, RefParams, UserCmd,
};
use serde::de::IntoDeserializer;

use crate::get_cs_delta_decoder_table;

use super::get_ghost::get_ghost;

pub mod bsp;

const DEMO_BUFFER_SIZE: [u8; 8] = [1, 0, 0, 0, 0, 0, 180, 66];
const DEFAULT_IN_SEQ: i32 = 1969;
const STEP_TIME: f32 = 0.3;

struct ServerFrame<'a> {
    /// Must include null terminator
    game_dir: &'a [u8],
    /// Must include null terminator
    host_name: Option<&'a [u8]>,
    /// Must include null terminator. Path must be full path relatively from game
    ///
    /// E.g.: maps/rvp_tundra-bhop.bsp
    map_file_name: &'a [u8],
}

/// Including SvcServerInfo SvcDeltaDescription SvcSetView SvcNewMovevars
fn insert_server(demo: &mut Demo, server_frame: ServerFrame, seq: i32) {
    let mut new_netmsg_data = NetMsgData::new(seq);

    let server_info = SvcServerInfo {
        protocol: 48,
        spawn_count: 5, // ?
        map_checksum: 0,
        client_dll_hash: &[0u8; 16],
        max_players: 1,
        player_index: 0,
        is_deathmatch: 0,
        game_dir: server_frame.game_dir,
        hostname: server_frame.host_name.unwrap_or(b"Ghost Demo Replay\0"),
        map_file_name: server_frame.map_file_name,
        map_cycle: b"a\0", // must be null string
        unknown: 0u8,
    };
    let server_info = ServerInfo::write(server_info);

    let dds: Vec<u8> = get_cs_delta_msg!()
        .iter()
        .flat_map(|dd| DeltaDescription::write(dd.to_owned(), &DeltaDecoderTable::new()))
        .collect();

    let set_view = SvcSetView { entity_index: 1 }; // always 1
    let set_view = SetView::write(set_view);

    let new_movevars = SvcNewMoveVars {
        gravity: 800.,
        stop_speed: 75.,
        max_speed: 320.,
        spectator_max_speed: 500.,
        accelerate: 5.,
        airaccelerate: 10.,
        water_accelerate: 10.,
        friction: 4.,
        edge_friction: 2.,
        water_friction: 1.,
        ent_garvity: 1.,
        bounce: 1.,
        step_size: 18.,
        max_velocity: 2000.,
        z_max: 409600.,
        wave_height: 0.,
        footsteps: 1,
        roll_angle: 0.,
        roll_speed: -1.9721523e-31, // have to use these magic numbers to work
        sky_color: vec![-1.972168e-31, -1.972168e-31, 9.4e-44],
        sky_vec: vec![-0.0, 2.68e-43, 2.7721908e20],
        sky_name: &[0],
    };
    let new_movevars = NewMovevars::write(new_movevars);

    new_netmsg_data.msg = [server_info, dds, set_view, new_movevars].concat().leak();
    println!("{:?}", new_netmsg_data.msg);

    let netmsg_framedata = FrameData::NetMsg((NetMsgFrameType::Start, new_netmsg_data));
    let netmsg_frame = Frame {
        time: 0.,
        frame: 0,
        data: netmsg_framedata,
    };

    demo.directory.entries[0].frames.insert(0, netmsg_frame);
    // demo.directory.entries[0].frame_count += 1;
}

/// `seq` is again to make sure things don't crash. Very sad.
fn insert_resourcelist(demo: &mut Demo, seq: i32) {
    // Resource list is counted by index
    // bsp file index must be 1
    // 1 bsp
    // 0 = usp model
    // 2-5 = pl_step

    let bsp = Resource {
        type_: nbit_num!(ResourceType::Model, 4),
        name: nbit_str!("maps/rvp_tundra-bhop.bsp\0"),
        index: nbit_num!(1, 12),
        size: nbit_num!(0, 3 * 8),
        flags: nbit_num!(1, 3),
        md5_hash: None,
        has_extra_info: false,
        extra_info: None,
    };

    let v_usp = Resource {
        type_: nbit_num!(ResourceType::Skin, 4),
        name: nbit_str!("models/v_usp.mdl\0"),
        index: nbit_num!(0, 12),
        size: nbit_num!(0, 3 * 8),
        flags: nbit_num!(0, 3),
        md5_hash: None,
        has_extra_info: false,
        extra_info: None,
    };

    let pl_steps: Vec<Resource> = (1..=4)
        .map(|i| Resource {
            type_: nbit_num!(ResourceType::Sound, 4),
            name: nbit_str!(format!("player/pl_step{}.wav\0", i)),
            index: nbit_num!(i + 1, 12), // remember to increment
            size: nbit_num!(0, 3 * 8),
            // TODO not sure what the flag does
            flags: nbit_num!(0, 3),
            md5_hash: None,
            has_extra_info: false,
            extra_info: None,
        })
        .collect();

    // add resources here
    let resources = [vec![bsp, v_usp], pl_steps].concat();

    let resource_list = SvcResourceList {
        resource_count: nbit_num!(resources.len(), 12),
        resources,
        consistencies: vec![],
    };

    let resource_list = ResourceList::write(resource_list);

    let mut new_netmsg_data = NetMsgData::new(seq);
    new_netmsg_data.msg = resource_list.leak();

    let netmsg_framedata = FrameData::NetMsg((NetMsgFrameType::Start, new_netmsg_data));
    let netmsg_frame = Frame {
        time: 0.,
        frame: 0,
        data: netmsg_framedata,
    };

    // demo.directory.entries[0].frames.push(netmsg_frame);
    demo.directory.entries[0].frames.insert(1, netmsg_frame);
    // demo.directory.entries[0].frame_count += 1;
}

fn insert_baseline(demo: &mut Demo, seq: i32) {
    let bsp = EntityS {
        entity_index: 0, // worldspawn is index 0
        index: nbit_num!(0, 11),
        type_: nbit_num!(1, 2),
        delta: Delta::from([
            ("movetype\0".to_owned(), vec![7, 0, 0, 0]),
            ("modelindex\0".to_owned(), vec![1, 0, 0, 0]), // but modelindex is 1
            ("solid\0".to_owned(), vec![4, 0]),
        ]),
    };

    let entities = vec![bsp];

    // max_client should be 1 because we are playing demo and it is OK.
    let spawn_baseline = SvcSpawnBaseline {
        entities,
        total_extra_data: nbit_num!(0, 6),
        extra_data: vec![],
    };
    let spawn_baseline =
        SpawnBaseline::write(spawn_baseline, &mut get_cs_delta_decoder_table!(), 1);

    let sign_on_num = SvcSignOnNum { sign: 1 };
    let sign_on_num = SignOnNum::write(sign_on_num);

    let mut new_netmsg_data = NetMsgData::new(seq);
    new_netmsg_data.msg = [spawn_baseline, sign_on_num].concat().leak();

    let netmsg_framedata = FrameData::NetMsg((NetMsgFrameType::Start, new_netmsg_data));
    let netmsg_frame = Frame {
        time: 0.,
        frame: 0,
        data: netmsg_framedata,
    };

    demo.directory.entries[0].frames.insert(3, netmsg_frame);
}

fn isolated_case(demo: &mut Demo) {
    let (mut delta_decoders, mut custom_messages) = init_parse!(demo);
    if let FrameData::NetMsg((_, data)) = &mut demo.directory.entries[0].frames[0].data {
        let (_, messages) =
            parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

        let mut messages: Vec<Message<'_>> = messages
            .into_iter()
            .filter(|msg| {
                !matches!(
                    msg,
                    Message::EngineMessage(
                        EngineMessage::SvcDeltaDescription(_)
                            | EngineMessage::SvcServerInfo(_)
                            | EngineMessage::SvcSetView(_)
                            | EngineMessage::SvcNewMovevars(_)
                            | EngineMessage::SvcNewUserMsg(_)
                            | EngineMessage::SvcSendExtraInfo(_)
                            | EngineMessage::SvcCdTrack(_)
                            | EngineMessage::SvcStuffText(_) // | EngineMessage::SvcUpdateUserInfo(_)
                            | EngineMessage::SvcUpdateUserInfo(_) // | EngineMessage::SvcSetView(_)
                                                                  // | EngineMessage::SvcSetView(_)
                    ) // | Message::UserMessage(_)
                )
            })
            .collect();

        let server_frame = ServerFrame {
            game_dir: b"cstrike\0",
            host_name: None,
            map_file_name: b"maps/rvp_tundra-bhop.bsp\0",
        };

        let server_info = SvcServerInfo {
            protocol: 48,
            spawn_count: 5, // ?
            map_checksum: 0,
            client_dll_hash: &[0u8; 16],
            max_players: 1,
            player_index: 0,
            is_deathmatch: 0,
            game_dir: server_frame.game_dir,
            hostname: server_frame.host_name.unwrap_or(b"Ghost Demo Replay\0"),
            map_file_name: server_frame.map_file_name,
            map_cycle: &[
                100, 101, 95, 97, 105, 114, 115, 116, 114, 105, 112, 13, 10, 99, 115, 95, 104, 97,
                118, 97, 110, 97, 13, 10, 100, 101, 95, 99, 104, 97, 116, 101, 97, 117, 13, 10,
                100, 101, 95, 97, 122, 116, 101, 99, 13, 10, 97, 115, 95, 111, 105, 108, 114, 105,
                103, 13, 10, 99, 115, 95, 115, 105, 101, 103, 101, 13, 10, 100, 101, 95, 99, 98,
                98, 108, 101, 13, 10, 100, 101, 95, 100, 117, 115, 116, 13, 10, 99, 115, 95, 55,
                52, 55, 13, 10, 100, 101, 95, 112, 114, 111, 100, 105, 103, 121, 13, 10, 99, 115,
                95, 97, 115, 115, 97, 117, 108, 116, 13, 10, 99, 115, 95, 111, 102, 102, 105, 99,
                101, 13, 10, 99, 115, 95, 105, 116, 97, 108, 121, 13, 10, 99, 115, 95, 98, 97, 99,
                107, 97, 108, 108, 101, 121, 13, 10, 99, 115, 95, 109, 105, 108, 105, 116, 105, 97,
                13, 10, 100, 101, 95, 116, 114, 97, 105, 110, 13, 10, 13, 10, 13, 10, 13, 10, 13,
                10, 0,
            ], // must be null string
            unknown: 0u8,
        };

        messages.insert(0, wrap_message!(SvcServerInfo, server_info));

        for m in get_cs_delta_msg!() {
            messages.insert(1, wrap_message!(SvcDeltaDescription, m));
        }

        let new_movevars = SvcNewMoveVars {
            gravity: 800.,
            stop_speed: 75.,
            max_speed: 320.,
            spectator_max_speed: 500.,
            accelerate: 5.,
            airaccelerate: 10.,
            water_accelerate: 10.,
            friction: 4.,
            edge_friction: 2.,
            water_friction: 1.,
            ent_garvity: 1.,
            bounce: 1.,
            step_size: 18.,
            max_velocity: 2000.,
            z_max: 409600.,
            wave_height: 0.,
            footsteps: 1,
            roll_angle: 0.,
            roll_speed: -1.9721523e-31, // have to use these magic numbers to work
            sky_color: vec![-1.972168e-31, -1.972168e-31, 9.4e-44],
            sky_vec: vec![-0.0, 2.68e-43, 2.7721908e20],
            sky_name: &[0],
        };
        messages.insert(8, wrap_message!(SvcNewMovevars, new_movevars));

        let set_view = SvcSetView { entity_index: 1 }; // always 1
        messages.insert(9, wrap_message!(SvcSetView, set_view));

        let write = write_netmsg(messages, &delta_decoders, &custom_messages);

        data.msg = write.leak();
    }

    // demo.directory.entries[0].frame_count -= 1;
    // demo.directory.entries[0].frames.remove(2);
}

pub fn insert_ghost(
    demo: &mut Demo,
    ghost_file_name: &str,
    override_frametime: Option<f32>,
    override_fov: Option<f32>,
) {
    // setup
    let ghost_info = get_ghost(ghost_file_name, &0.);
    // let (delta_decoders, custom_messages) = init_parse!(demo);

    // removing_msg(demo);

    demo.directory.entries[0].frames.remove(1);
    insert_resourcelist(demo, 2);

    demo.directory.entries[0].frames.remove(0);
    insert_server(
        demo,
        ServerFrame {
            game_dir: b"cstrike\0",
            host_name: None,
            map_file_name: b"maps/rvp_tundra-bhop.bsp\0",
        },
        3,
    );

    demo.directory.entries[0].frames.remove(3);
    insert_baseline(demo, 4);

    // set directory entry info
    let entry1 = &mut demo.directory.entries[1];
    entry1.frame_count = ghost_info.frames.len() as i32;

    // nuke
    entry1.frames.clear();

    // some tracking stuffs
    let mut time = 0.;
    let mut time_step = STEP_TIME;
    let mut last_pos: [f32; 3] = [0.; 3];
    let mut last_z_vel = 0.;

    // begin :DDD
    // 1 0 Frame { time: 0.0, frame: 0, data: DemoStart }
    let start_framedata = FrameData::DemoStart;
    let start_frame = Frame {
        time,
        frame: 0,
        data: start_framedata,
    };
    entry1.frames.push(start_frame);

    // insert :DDD
    for (frame_idx, frame) in ghost_info.frames.iter().enumerate() {
        let frametime = override_frametime.unwrap_or(if ghost_info.use_frametime {
            frame.frametime
        } else {
            unreachable!("There is no given frametime.")
        });

        let fov = override_fov.unwrap_or(90.);

        // buffer because it does so.... not sure the number for now :DDD
        let buffer_framedata = FrameData::DemoBuffer(DemoBufferData {
            buffer: &DEMO_BUFFER_SIZE,
        });
        let buffer_frame = Frame {
            time,
            frame: (frame_idx + 1) as i32,
            data: buffer_framedata,
        };

        // client data
        let clientdata_framedata = FrameData::ClientData(ClientDataData {
            origin: frame.origin,
            viewangles: frame.viewangles,
            weapon_bits: 0,
            fov,
        });
        let clientdata_frame = Frame {
            time,
            frame: (frame_idx + 1) as i32,
            data: clientdata_framedata,
        };

        // netmsg
        let mut new_netmsg_data = NetMsgData::new(DEFAULT_IN_SEQ + frame_idx as i32);
        new_netmsg_data.info.ref_params.vieworg = frame.origin;
        new_netmsg_data.info.ref_params.viewangles = frame.viewangles;
        new_netmsg_data.info.ref_params.frametime = frametime;
        new_netmsg_data.info.ref_params.time = time;
        new_netmsg_data.info.ref_params.simorg = frame.origin;
        new_netmsg_data.info.ref_params.cl_viewangles = frame.viewangles;
        new_netmsg_data.info.usercmd.viewangles = frame.viewangles;
        // new_netmsg_data.info.movevars.sky_name = hehe; // TODO... DO NOT ASSIGN to &[]
        new_netmsg_data.info.view = frame.origin;

        let speed = ((frame.origin[0] - last_pos[0]).powi(2)
            + (frame.origin[1] - last_pos[1]).powi(2))
        .sqrt()
            / frametime;
        let curr_z_vel = (frame.origin[2] - last_pos[2]) / frametime;

        // if speed is less than 150 then increase time_step
        if speed < 150. {
            time_step = STEP_TIME + 0.1;
        }

        // play jump sound
        if let Some(buttons) = frame.buttons {
            if buttons & Buttons::Jump as u32 != 0 && curr_z_vel > last_z_vel && speed > 150. {
                let svcsound = SvcSound {
                    flags: bitvec![u8, Lsb0; 1, 1, 1, 0, 0, 0, 0, 0, 0].into(),
                    volume: nbit_num!(128, 8).into(),
                    attenuation: nbit_num!(204, 8).into(),
                    channel: nbit_num!(5, 3).into(),
                    entity_index: nbit_num!(1, 11).into(),
                    sound_index_long: nbit_num!(rand_int_range!(2, 5), 16).into(),
                    sound_index_short: None,
                    has_x: true,
                    has_y: true,
                    has_z: true,
                    origin_x: Some(OriginCoord {
                        int_flag: true,
                        fraction_flag: false,
                        is_negative: frame.origin[0].is_sign_negative().into(),
                        int_value: nbit_num!(frame.origin[0].round().abs() as i32, 12).into(),
                        fraction_value: None,
                    }),
                    origin_y: Some(OriginCoord {
                        int_flag: true,
                        fraction_flag: false,
                        is_negative: frame.origin[1].is_sign_negative().into(),
                        int_value: nbit_num!(frame.origin[1].round().abs() as i32, 12).into(),
                        fraction_value: None,
                    }),
                    origin_z: Some(OriginCoord {
                        int_flag: true,
                        fraction_flag: false,
                        is_negative: frame.origin[2].is_sign_negative().into(),
                        int_value: nbit_num!(frame.origin[2].round().abs() as i32, 12).into(),
                        fraction_value: None,
                    }),
                    pitch: bitvec![u8, Lsb0; 1, 0, 0, 0, 0, 0, 0, 0].into(),
                };

                let svcsound_msg = Sound::write(svcsound);

                new_netmsg_data.msg = [new_netmsg_data.msg.to_owned(), svcsound_msg]
                    .concat()
                    .leak();
            }
        }
        // play step sound every 0.3 on ground
        if time_step <= 0. && last_pos[2] == frame.origin[2] {
            time_step = STEP_TIME;

            // TODO do all the steps randomly
            let svcsound = SvcSound {
                flags: bitvec![u8, Lsb0; 1, 1, 1, 0, 0, 0, 0, 0, 0].into(),
                volume: nbit_num!(128, 8).into(),
                attenuation: nbit_num!(204, 8).into(),
                channel: nbit_num!(5, 3).into(),
                entity_index: nbit_num!(1, 11).into(),
                sound_index_long: nbit_num!(rand_int_range!(2, 5), 16).into(),
                sound_index_short: None,
                has_x: true,
                has_y: true,
                has_z: true,
                origin_x: Some(OriginCoord {
                    int_flag: true,
                    fraction_flag: false,
                    is_negative: frame.origin[0].is_sign_negative().into(),
                    int_value: nbit_num!(frame.origin[0].round().abs() as i32, 12).into(),
                    fraction_value: None,
                }),
                origin_y: Some(OriginCoord {
                    int_flag: true,
                    fraction_flag: false,
                    is_negative: frame.origin[1].is_sign_negative().into(),
                    int_value: nbit_num!(frame.origin[1].round().abs() as i32, 12).into(),
                    fraction_value: None,
                }),
                origin_z: Some(OriginCoord {
                    int_flag: true,
                    fraction_flag: false,
                    is_negative: frame.origin[2].is_sign_negative().into(),
                    int_value: nbit_num!(frame.origin[2].round().abs() as i32, 12).into(),
                    fraction_value: None,
                }),
                pitch: nbit_num!(1, 8).into(),
            };

            let svcsound_msg = Sound::write(svcsound);

            new_netmsg_data.msg = [new_netmsg_data.msg.to_owned(), svcsound_msg]
                .concat()
                .leak();
        }

        let netmsg_framedata = FrameData::NetMsg((NetMsgFrameType::Normal, new_netmsg_data));
        let netmsg_frame = Frame {
            time,
            frame: (frame_idx + 1) as i32,
            data: netmsg_framedata,
        };

        // insert
        entry1
            .frames
            .append(&mut vec![buffer_frame, clientdata_frame, netmsg_frame]);

        time += frametime;
        time_step -= frametime;
        last_pos = frame.origin;
    }

    // demo section end :DD
    // 1 388 Frame { time: 1.260376, frame: 126, data: NextSection }
    let end_framedata = FrameData::DemoStart;
    let end_frame = Frame {
        time,
        frame: ghost_info.frames.len() as i32,
        data: end_framedata,
    };
    entry1.frames.push(end_frame);
}
