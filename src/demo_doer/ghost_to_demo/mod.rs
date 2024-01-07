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
use demosuperimpose_goldsrc::{
    init_parse, nbit_num, nbit_str,
    netmsg_doer::{
        parse_netmsg, parse_netmsg_immutable, resource_list::ResourceList, write_netmsg, NetMsgDoer,
    },
    types::Resource,
    types::{EngineMessage, Message, SvcDeltaDescription, SvcResourceList},
    utils::{get_cs_delta_msg, NetMsgDataMethods, ResourceType},
};
use hldemo::{
    parse::frame, ClientDataData, Demo, DemoBufferData, Frame, FrameData, MoveVars, NetMsgData,
    NetMsgFrameType, NetMsgInfo, RefParams, UserCmd,
};

use super::get_ghost::get_ghost;

pub mod bsp;

const DEMO_BUFFER_SIZE: [u8; 8] = [1, 0, 0, 0, 0, 0, 180, 66];
const DEFAULT_IN_SEQ: i32 = 1969;

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
    // Resource list
    // 0 = usp model
    // 1-4 = pl_step
    // TODO it is possible to include texture in here as well

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
            index: nbit_num!(i, 12),
            size: nbit_num!(0, 3 * 8),
            // TODO not sure what the flag does
            flags: nbit_num!(0, 3),
            md5_hash: None,
            has_extra_info: false,
            extra_info: None,
        })
        .collect();

    let resource_list = SvcResourceList {
        resource_count: nbit_num!(1 + 4, 12), // remember to increase accordingly
        resources: [vec![v_usp], pl_steps].concat(),
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
}

/// checking out what can be deleted
fn removing_msg(demo: &mut Demo) {
    let (mut delta_decoders, mut custom_messages) = init_parse!(demo);

    if let FrameData::NetMsg((_, data)) = &mut demo.directory.entries[0].frames[0].data {
        let (_, messages) =
            parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

        let messages: Vec<Message<'_>> = messages
            .into_iter()
            .filter(|msg| {
                !matches!(
                    msg,
                    Message::EngineMessage(
                        EngineMessage::SvcCustomization(_)
                            | EngineMessage::SvcUpdateUserInfo(_)
                            | EngineMessage::SvcTime(_)
                            | EngineMessage::SvcClientData(_)
                            | EngineMessage::SvcLightStyle(_)
                            | EngineMessage::SvcResourceRequest(_)
                            | EngineMessage::SvcSendExtraInfo(_)
                            | EngineMessage::SvcNewMovevars(_)
                            | EngineMessage::SvcCdTrack(_)
                            | EngineMessage::SvcNewUserMsg(_)
                            | EngineMessage::SvcStuffText(_)
                    ) | Message::UserMessage(_)
                )
            })
            .collect();

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

    // set directory entry info
    let entry1 = &mut demo.directory.entries[1];
    entry1.frame_count = ghost_info.frames.len() as i32;

    // nuke
    entry1.frames.clear();

    // for end frame
    let mut time = 0.;

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
