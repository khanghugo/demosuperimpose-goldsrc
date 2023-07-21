use hldemo::{Demo, Directory, DirectoryEntry, Frame, FrameData, Header};
use std::fs;
use std::io::Write;

pub struct DemoWriter {
    pub filename: String,
    data: Vec<u8>,
    offset: usize,
    has_written_next_section: bool,
}

impl DemoWriter {
    pub fn new(filename: String) -> DemoWriter {
        DemoWriter {
            filename,
            data: Vec::new(),
            offset: 0,
            has_written_next_section: false,
        }
    }

    pub fn write_file(&mut self, demo: Demo) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.filename)
            .unwrap();

        self.write_demo(demo);
        // println!("{:?}", self.data);
        let _ = file.write_all(&self.data);
    }

    fn write_demo(&mut self, demo: Demo) {
        // magic has 8 bytes in total
        self.append_u8_slice("HLDEMO\x00\x00".as_bytes());

        self.write_header(demo.header);

        for entry in &demo.directory.entries {
            for frame in &entry.frames {
                self.has_written_next_section = false;
                self.write_frame(frame);
            }

            if !self.has_written_next_section {
                self.append_u8(5u8);
                self.append_i32(0i32);
                self.append_i32(0i32);
                self.has_written_next_section = true;
            }
        }

        self.append_u32(demo.directory.entries.len() as u32);

        for entry in demo.directory.entries {
            self.write_directory_entry(entry);
        }
    }

    fn write_header(&mut self, header: Header) {
        self.append_i32(header.demo_protocol);
        self.append_i32(header.net_protocol);
        self.append_u8_slice(header.map_name);
        self.append_u8_slice(header.game_dir);
        self.append_u32(header.map_crc);
        self.append_i32(header.directory_offset);
    }

    // fn write_directory(&mut self, directory: Directory) {
    //     for entry in directory.entries {}
    // }

    fn write_directory_entry(&mut self, entry: DirectoryEntry) {
        self.append_i32(entry.entry_type);
        self.append_u8_slice(entry.description);
        self.append_i32(entry.flags);
        self.append_i32(entry.cd_track);
        self.append_f32(entry.track_time);
        self.append_i32(entry.frame_count);
        self.append_i32(entry.offset);
        self.append_i32(entry.file_length);
    }

    fn write_frame(&mut self, frame: &Frame) {
        match frame.data {
            FrameData::DemoStart => self.append_u8(2u8),
            FrameData::ConsoleCommand(_) => self.append_u8(3u8),
            FrameData::ClientData(_) => self.append_u8(4u8),
            FrameData::NextSection => {
                self.append_u8(5u8);
                self.has_written_next_section = true;
            }
            FrameData::Event(_) => self.append_u8(6u8),
            FrameData::WeaponAnim(_) => self.append_u8(7u8),
            FrameData::Sound(_) => self.append_u8(8u8),
            FrameData::DemoBuffer(_) => self.append_u8(9u8),
            FrameData::NetMsg(_) => self.append_u8(0u8),
        }

        self.append_f32(frame.time);
        self.append_i32(frame.frame);
        self.write_frame_data(&frame.data);
    }
    fn write_frame_data(&mut self, frame: &FrameData) {
        match frame {
            FrameData::DemoStart => (),
            FrameData::ConsoleCommand(frame) => self.append_u8_slice(frame.command),
            FrameData::ClientData(frame) => {
                self.append_f32_array(frame.origin);
                self.append_f32_array(frame.viewangles);
                self.append_i32(frame.weapon_bits);
                self.append_f32(frame.fov);
            }
            FrameData::NextSection => (),
            FrameData::Event(frame) => {
                self.append_i32(frame.flags);
                self.append_i32(frame.index);
                self.append_f32(frame.delay);

                self.append_i32(frame.args.flags);
                self.append_i32(frame.args.entity_index);
                self.append_f32_array(frame.args.origin);
                self.append_f32_array(frame.args.angles);
                self.append_f32_array(frame.args.velocity);
                self.append_i32(frame.args.ducking);
                self.append_f32(frame.args.fparam1);
                self.append_f32(frame.args.fparam2);
                self.append_i32(frame.args.iparam1);
                self.append_i32(frame.args.iparam2);
                self.append_i32(frame.args.bparam1);
                self.append_i32(frame.args.bparam2);
            }
            FrameData::WeaponAnim(frame) => {
                self.append_i32(frame.anim);
                self.append_i32(frame.body);
            }
            FrameData::Sound(frame) => {
                self.append_i32(frame.channel);
                self.append_u32(frame.sample.len() as u32);
                self.append_u8_slice(frame.sample);
                self.append_f32(frame.attenuation);
                self.append_f32(frame.volume);
                self.append_i32(frame.flags);
                self.append_i32(frame.pitch);
            }
            FrameData::DemoBuffer(frame) => {
                self.append_u32(frame.buffer.len() as u32);
                self.append_u8_slice(frame.buffer);
            }
            FrameData::NetMsg((type_, data)) => {
                self.append_f32(data.info.timestamp);
                // ref_params
                self.append_f32_array(data.info.ref_params.vieworg);
                self.append_f32_array(data.info.ref_params.viewangles);
                self.append_f32_array(data.info.ref_params.forward);
                self.append_f32_array(data.info.ref_params.right);
                self.append_f32_array(data.info.ref_params.up);
                self.append_f32(data.info.ref_params.frametime);
                self.append_f32(data.info.ref_params.time);
                self.append_i32(data.info.ref_params.intermission);
                self.append_i32(data.info.ref_params.paused);
                self.append_i32(data.info.ref_params.spectator);
                self.append_i32(data.info.ref_params.onground);
                self.append_i32(data.info.ref_params.waterlevel);
                self.append_f32_array(data.info.ref_params.simvel);
                self.append_f32_array(data.info.ref_params.simorg);
                self.append_f32_array(data.info.ref_params.viewheight);
                self.append_f32(data.info.ref_params.idealpitch);
                self.append_f32_array(data.info.ref_params.cl_viewangles);
                self.append_i32(data.info.ref_params.health);
                self.append_f32_array(data.info.ref_params.crosshairangle);
                self.append_f32(data.info.ref_params.viewsize);
                self.append_f32_array(data.info.ref_params.punchangle);
                self.append_i32(data.info.ref_params.maxclients);
                self.append_i32(data.info.ref_params.viewentity);
                self.append_i32(data.info.ref_params.playernum);
                self.append_i32(data.info.ref_params.max_entities);
                self.append_i32(data.info.ref_params.demoplayback);
                self.append_i32(data.info.ref_params.hardware);
                self.append_i32(data.info.ref_params.smoothing);
                self.append_i32(data.info.ref_params.ptr_cmd);
                self.append_i32(data.info.ref_params.ptr_movevars);
                self.append_i32_array_4(data.info.ref_params.viewport);
                self.append_i32(data.info.ref_params.next_view);
                self.append_i32(data.info.ref_params.only_client_draw);
                // usercmd
                self.append_i16(data.info.usercmd.lerp_msec);
                self.append_u8(data.info.usercmd.msec);
                self.append_u8(0u8); // unknown
                self.append_f32_array(data.info.usercmd.viewangles);
                self.append_f32(data.info.usercmd.forwardmove);
                self.append_f32(data.info.usercmd.sidemove);
                self.append_f32(data.info.usercmd.upmove);
                self.append_i8(data.info.usercmd.lightlevel);
                self.append_u8(0u8); // unknown
                self.append_u16(data.info.usercmd.buttons);
                self.append_i8(data.info.usercmd.impulse);
                self.append_i8(data.info.usercmd.weaponselect);
                self.append_u8(0u8); // unknown
                self.append_u8(0u8); // unknown
                self.append_i32(data.info.usercmd.impact_index);
                self.append_f32_array(data.info.usercmd.impact_position);
                // movevars
                self.append_f32(data.info.movevars.gravity);
                self.append_f32(data.info.movevars.stopspeed);
                self.append_f32(data.info.movevars.maxspeed);
                self.append_f32(data.info.movevars.spectatormaxspeed);
                self.append_f32(data.info.movevars.accelerate);
                self.append_f32(data.info.movevars.airaccelerate);
                self.append_f32(data.info.movevars.wateraccelerate);
                self.append_f32(data.info.movevars.friction);
                self.append_f32(data.info.movevars.edgefriction);
                self.append_f32(data.info.movevars.waterfriction);
                self.append_f32(data.info.movevars.entgravity);
                self.append_f32(data.info.movevars.bounce);
                self.append_f32(data.info.movevars.stepsize);
                self.append_f32(data.info.movevars.maxvelocity);
                self.append_f32(data.info.movevars.zmax);
                self.append_f32(data.info.movevars.wave_height);
                self.append_i32(data.info.movevars.footsteps);
                self.append_u8_slice(data.info.movevars.sky_name);
                self.append_f32(data.info.movevars.rollangle);
                self.append_f32(data.info.movevars.rollspeed);
                self.append_f32(data.info.movevars.skycolor_r);
                self.append_f32(data.info.movevars.skycolor_g);
                self.append_f32(data.info.movevars.skycolor_b);
                self.append_f32(data.info.movevars.skyvec_x);
                self.append_f32(data.info.movevars.skyvec_y);
                self.append_f32(data.info.movevars.skyvec_z);
                // still in info
                self.append_f32_array(data.info.view);
                self.append_i32(data.info.viewmodel);
                // now other data
                self.append_i32(data.incoming_sequence);
                self.append_i32(data.incoming_acknowledged);
                self.append_i32(data.incoming_reliable_acknowledged);
                self.append_i32(data.incoming_reliable_sequence);
                self.append_i32(data.outgoing_sequence);
                self.append_i32(data.reliable_sequence);
                self.append_i32(data.last_reliable_sequence);
                self.append_u32(data.msg.len() as u32);
                self.append_u8_slice(data.msg);
            }
        }
    }

    fn append_u32(&mut self, i: u32) {
        self.data.extend(i.to_le_bytes());
        self.offset += 4;
    }

    fn append_i32(&mut self, i: i32) {
        self.data.extend(i.to_le_bytes());
        self.offset += 4;
    }

    fn append_f32(&mut self, i: f32) {
        self.data.extend(i.to_le_bytes());
        self.offset += 4;
    }

    fn append_u8(&mut self, i: u8) {
        self.data.extend(i.to_le_bytes());
        self.offset += 1;
    }

    fn append_i8(&mut self, i: i8) {
        self.data.extend(i.to_le_bytes());
        self.offset += 1;
    }

    fn append_u16(&mut self, i: u16) {
        self.data.extend(i.to_le_bytes());
        self.offset += 2;
    }

    fn append_i16(&mut self, i: i16) {
        self.data.extend(i.to_le_bytes());
        self.offset += 2;
    }

    fn append_u8_slice(&mut self, i: &[u8]) {
        self.data.extend_from_slice(i);
        self.offset += i.len();
    }

    fn append_f32_array(&mut self, i: [f32; 3]) {
        self.data
            .extend(i.iter().map(|x| x.to_le_bytes()).flatten());
        self.offset += 4 * 3;
    }

    fn append_i32_array_4(&mut self, i: [i32; 4]) {
        self.data
            .extend(i.iter().map(|x| x.to_le_bytes()).flatten());
        self.offset += 4 * 4;
    }
}
