// bsp info to insert
// ghost demo to insert

use demosuperimpose_goldsrc::{
    init_parse,
    netmsg_doer::{parse_netmsg_immutable, write_netmsg},
};
use hldemo::{
    parse::frame, ClientDataData, Demo, DemoBufferData, Frame, FrameData, MoveVars, NetMsgData,
    NetMsgFrameType, NetMsgInfo, RefParams, UserCmd,
};

use super::get_ghost::get_ghost;

mod bsp;

// demo buffer bytes presumable little endian [1, 0, 0, 0, 0, 0, 180, 66]
const DEMO_BUFFER_SIZE: [u8; 8] = [1, 0, 0, 0, 0, 0, 180, 66];
const VEC_0: [f32; 3] = [0., 0., 0.];
const VIEWHEIGHT: [f32; 3] = [0.0, 0.0, 17.0];
const VIEWPORT: [i32; 4] = [0, 0, 1024, 768];
const DEFAULT_IN_SEQ: i32 = 1969;

// TODO: for now only singular please
// override frametime will force uniform frametime for all frames.
pub fn insert_ghost(
    demo: &mut Demo,
    ghost_file_name: &str,
    override_frametime: Option<f32>,
    override_fov: Option<f32>,
) {
    // setup
    let ghost_info = get_ghost(ghost_file_name, &0.);
    // let (delta_decoders, custom_messages) = init_parse!(demo);

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
        let sky_name_bind = vec![0u8; 32];
        let netmsg_framedata = FrameData::NetMsg((
            NetMsgFrameType::Normal,
            NetMsgData {
                info: NetMsgInfo {
                    timestamp: 0.0,
                    ref_params: RefParams {
                        vieworg: frame.origin,
                        viewangles: frame.viewangles,
                        forward: VEC_0,
                        right: VEC_0,
                        up: VEC_0,
                        frametime,
                        time,
                        intermission: 0,
                        paused: 0,
                        spectator: 0,
                        onground: 0,
                        waterlevel: 0,
                        simvel: VEC_0,
                        simorg: frame.origin,
                        viewheight: VIEWHEIGHT,
                        idealpitch: 0.,
                        cl_viewangles: frame.viewangles,
                        health: 100,
                        crosshairangle: VEC_0,
                        viewsize: 120.,
                        punchangle: VEC_0,
                        maxclients: 32,
                        viewentity: 1,
                        playernum: 0,
                        max_entities: 6969,
                        demoplayback: 0,
                        hardware: 1,
                        smoothing: 1,
                        ptr_cmd: 0,
                        ptr_movevars: 0,
                        viewport: VIEWPORT,
                        next_view: 0,
                        only_client_draw: 0,
                    },
                    usercmd: UserCmd {
                        lerp_msec: 9,
                        msec: 10,
                        viewangles: frame.viewangles,
                        forwardmove: 0.,
                        sidemove: 0.,
                        upmove: 0.,
                        lightlevel: 68,
                        buttons: 0,
                        impulse: 0,
                        weaponselect: 0,
                        impact_index: 0,
                        impact_position: VEC_0,
                    },
                    movevars: MoveVars {
                        gravity: 800.0,
                        stopspeed: 75.0,
                        maxspeed: 320.,
                        spectatormaxspeed: 500.,
                        accelerate: 5.,
                        airaccelerate: 10.,
                        wateraccelerate: 10.,
                        friction: 4.,
                        edgefriction: 2.,
                        waterfriction: 1.,
                        entgravity: 1.,
                        bounce: 1.,
                        stepsize: 1.,
                        maxvelocity: 2000.,
                        zmax: 409600.,
                        wave_height: 0.,
                        footsteps: 1,
                        sky_name: sky_name_bind.leak(), // TODO
                        rollangle: 0.,
                        rollspeed: 0.,
                        skycolor_r: 0.,
                        skycolor_g: 0.,
                        skycolor_b: 0.,
                        skyvec_x: 0.,
                        skyvec_y: 0.,
                        skyvec_z: 0.,
                    },
                    view: frame.origin,
                    viewmodel: 256,
                },
                // in seq
                // in ack = in seq - 1
                // incoming_reliable_acknowledged = 1
                // incoming_reliable_sequence = 0
                // out seq = in seq
                // reliable_sequence = 1
                // last_reliable_sequence < in seq
                incoming_sequence: DEFAULT_IN_SEQ,
                incoming_acknowledged: DEFAULT_IN_SEQ - 1,
                incoming_reliable_acknowledged: 1,
                incoming_reliable_sequence: 0,
                outgoing_sequence: DEFAULT_IN_SEQ,
                reliable_sequence: 1,
                last_reliable_sequence: DEFAULT_IN_SEQ - 69,
                msg: &[],
            },
        ));
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
