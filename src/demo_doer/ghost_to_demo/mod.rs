// bsp info to insert
// ghost demo to insert

/// frame 0: SvcServerInfo SvcDeltaDescription
///
/// SvcSetView might or might not be necessary if we don't cache model. Without it, the player model moves just fine
/// but view origin is detached. If included, set to player index = 1. In another word, it would hide player model.
///
/// frame 1: SvcResourceList
/// frame 2: 8 nopes, omittable
/// frame 3: SvcSpawnBaseline, SvcSignOnNum(_) = 1,
/// frame 4: svcpackent entity
///
use bitvec::bitvec;
use bitvec::prelude::*;

use demosuperimpose_goldsrc::netmsg_doer::sound::Sound;
use demosuperimpose_goldsrc::rand_int_range;
use demosuperimpose_goldsrc::types::OriginCoord;
use demosuperimpose_goldsrc::utils::Buttons;
use demosuperimpose_goldsrc::{
    init_parse, nbit_num, nbit_str,
    netmsg_doer::{
        parse_netmsg, parse_netmsg_immutable, resource_list::ResourceList, write_netmsg, NetMsgDoer,
    },
    types::Resource,
    types::{EngineMessage, Message, SvcDeltaDescription, SvcResourceList, SvcSound},
    utils::{get_cs_delta_msg, NetMsgDataMethods, ResourceType},
};
use hldemo::parse::frame::netmsg;
use hldemo::{
    parse::frame, ClientDataData, Demo, DemoBufferData, Frame, FrameData, MoveVars, NetMsgData,
    NetMsgFrameType, NetMsgInfo, RefParams, UserCmd,
};
use serde::de::IntoDeserializer;

use super::get_ghost::get_ghost;

pub mod bsp;

const DEMO_BUFFER_SIZE: [u8; 8] = [1, 0, 0, 0, 0, 0, 180, 66];
const DEFAULT_IN_SEQ: i32 = 1969;
const STEP_TIME: f32 = 0.3;

fn insert_delta_description(demo: &mut Demo, seq: i32) {
    let mut new_netmsg_data = NetMsgData::new(seq);
    new_netmsg_data.msg = get_cs_delta_msg().leak();

    let netmsg_framedata = FrameData::NetMsg((NetMsgFrameType::Start, new_netmsg_data));
    let netmsg_frame = Frame {
        time: 0.,
        frame: 0,
        data: netmsg_framedata,
    };

    demo.directory.entries[0].frames.push(netmsg_frame);
    demo.directory.entries[0].frame_count += 1;
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

    demo.directory.entries[0].frames.push(netmsg_frame);
    demo.directory.entries[0].frame_count += 1;

    // demo.directory.entries[0].frames[1] = netmsg_frame;
}

/// checking out what can be deleted
fn removing_msg(demo: &mut Demo) {
    let (mut delta_decoders, mut custom_messages) = init_parse!(demo);

    if let FrameData::NetMsg((_, data)) = &mut demo.directory.entries[0].frames[1].data {
        let (_, messages) =
            parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

        let mut messages: Vec<Message<'_>> = messages
            .into_iter()
            .filter(|msg| {
                !matches!(
                    msg,
                    Message::EngineMessage(
                        // EngineMessage::SvcCustomization(_)
                        //     | EngineMessage::SvcUpdateUserInfo(_)
                        //     | EngineMessage::SvcTime(_)
                        //     | EngineMessage::SvcClientData(_)
                        //     | EngineMessage::SvcLightStyle(_)
                        //     | EngineMessage::SvcResourceRequest(_)
                        //     | EngineMessage::SvcSendExtraInfo(_)
                        //     | EngineMessage::SvcNewMovevars(_)
                        //     | EngineMessage::SvcCdTrack(_)
                        //     | EngineMessage::SvcNewUserMsg(_)
                        //     | EngineMessage::SvcStuffText(_)
                            | EngineMessage::SvcResourceList(_)
                    ) | Message::UserMessage(_)
                )
            })
            .collect();

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

        let resource_list = SvcResourceList {
            resource_count: nbit_num!(6, 12), // remember to increase accordingly
            resources: [vec![bsp, v_usp], pl_steps].concat(),
            consistencies: vec![],
        };

        messages.push(Message::EngineMessage(EngineMessage::SvcResourceList(
            resource_list,
        )));

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

    removing_msg(demo);
    // insert_resourcelist(demo, 3);

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
            if buttons & Buttons::Jump as u32 != 0 && curr_z_vel > last_z_vel {
                if speed > 150. {
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
