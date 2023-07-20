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
        self.data.extend_from_slice("HLDEMO".as_bytes());
        self.data.extend_from_slice("\x00\x00".as_bytes());
        // self.append_u8_slice("HLDEMO\x00\x00".as_bytes());

        self.write_header(demo.header);

        for entry in &demo.directory.entries {
            for frame in &entry.frames {
                self.has_written_next_section = false;
                self.write_frame(frame);
            }

            if !self.has_written_next_section {
                self.data.extend(5u8.to_le_bytes());
                self.data.extend(0f32.to_be_bytes());
                self.data.extend(0i32.to_be_bytes());
                self.has_written_next_section = true;
            }
        }

        self.data
            .extend((demo.directory.entries.len() as u32).to_le_bytes());

        for entry in demo.directory.entries {
            self.write_directory_entry(entry);
        }
    }

    fn write_header(&mut self, header: Header) {
        self.data.extend(header.demo_protocol.to_le_bytes());
        self.data.extend(header.net_protocol.to_le_bytes());
        self.data.extend_from_slice(header.map_name);
        self.data.extend_from_slice(header.game_dir);
        self.data.extend(header.map_crc.to_le_bytes());
        self.data.extend(header.directory_offset.to_le_bytes());
    }

    // fn write_directory(&mut self, directory: Directory) {
    //     for entry in directory.entries {}
    // }

    fn write_directory_entry(&mut self, entry: DirectoryEntry) {
        self.data.extend(entry.entry_type.to_le_bytes());
        self.data.extend_from_slice(entry.description);
        self.data.extend(entry.flags.to_le_bytes());
        self.data.extend(entry.cd_track.to_le_bytes());
        self.data.extend(entry.track_time.to_le_bytes());
        self.data.extend(entry.frame_count.to_le_bytes());
        self.data.extend(entry.offset.to_le_bytes());
        self.data.extend(entry.file_length.to_le_bytes());
    }

    fn write_frame(&mut self, frame: &Frame) {
        match frame.data {
            FrameData::DemoStart => self.data.extend(2u8.to_le_bytes()),
            FrameData::ConsoleCommand(_) => self.data.extend(3u8.to_le_bytes()),
            FrameData::ClientData(_) => self.data.extend(4u8.to_le_bytes()),
            FrameData::NextSection => {
                self.data.extend(5u8.to_le_bytes());
                self.has_written_next_section = true;
            }
            FrameData::Event(_) => self.data.extend(6u8.to_le_bytes()),
            FrameData::WeaponAnim(_) => self.data.extend(7u8.to_le_bytes()),
            FrameData::Sound(_) => self.data.extend(8u8.to_le_bytes()),
            FrameData::DemoBuffer(_) => self.data.extend(9u8.to_le_bytes()),
            // For some reasons 0 loads faster than 1.
            FrameData::NetMsg(_) => self.data.extend(0u8.to_le_bytes()),
        }

        self.data.extend(frame.time.to_le_bytes());
        self.data.extend(frame.frame.to_le_bytes());
        self.write_frame_data(&frame.data);
    }
    fn write_frame_data(&mut self, frame: &FrameData) {
        match frame {
            FrameData::DemoStart => (),
            FrameData::ConsoleCommand(frame) => self.data.extend_from_slice(frame.command),
            FrameData::ClientData(frame) => {
                self.data
                    .extend(frame.origin.iter().map(|x| x.to_le_bytes()).flatten());
                self.data
                    .extend(frame.viewangles.iter().map(|x| x.to_le_bytes()).flatten());
                self.data.extend(frame.weapon_bits.to_le_bytes());
                self.data.extend(frame.fov.to_le_bytes());
            }
            FrameData::NextSection => (),
            FrameData::Event(frame) => {
                self.data.extend(frame.flags.to_le_bytes());
                self.data.extend(frame.index.to_le_bytes());
                self.data.extend(frame.delay.to_le_bytes());

                self.data.extend(frame.args.flags.to_le_bytes());
                self.data.extend(frame.args.entity_index.to_le_bytes());
                self.data
                    .extend(frame.args.origin.iter().map(|x| x.to_le_bytes()).flatten());
                self.data
                    .extend(frame.args.angles.iter().map(|x| x.to_le_bytes()).flatten());
                self.data.extend(
                    frame
                        .args
                        .velocity
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(frame.args.ducking.to_le_bytes());
                self.data.extend(frame.args.fparam1.to_le_bytes());
                self.data.extend(frame.args.fparam2.to_le_bytes());
                self.data.extend(frame.args.iparam1.to_le_bytes());
                self.data.extend(frame.args.iparam2.to_le_bytes());
                self.data.extend(frame.args.bparam1.to_le_bytes());
                self.data.extend(frame.args.bparam2.to_le_bytes());
            }
            FrameData::WeaponAnim(frame) => {
                self.data.extend(frame.anim.to_le_bytes());
                self.data.extend(frame.body.to_le_bytes());
            }
            FrameData::Sound(frame) => {
                self.data.extend(frame.channel.to_le_bytes());
                self.data.extend((frame.sample.len() as i32).to_le_bytes());
                self.data.extend_from_slice(frame.sample);
                self.data.extend(frame.attenuation.to_le_bytes());
                self.data.extend(frame.volume.to_le_bytes());
                self.data.extend(frame.flags.to_le_bytes());
                self.data.extend(frame.pitch.to_le_bytes());
            }
            FrameData::DemoBuffer(frame) => {
                self.data.extend((frame.buffer.len() as u32).to_le_bytes());
                self.data.extend_from_slice(frame.buffer);
            }
            FrameData::NetMsg((type_, data)) => {
                self.data.extend(data.info.timestamp.to_le_bytes());
                // ref_params
                self.data.extend(
                    data.info
                        .ref_params
                        .vieworg
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(
                    data.info
                        .ref_params
                        .viewangles
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(
                    data.info
                        .ref_params
                        .forward
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(
                    data.info
                        .ref_params
                        .right
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(
                    data.info
                        .ref_params
                        .up
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data
                    .extend(data.info.ref_params.frametime.to_le_bytes());
                self.data.extend(data.info.ref_params.time.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.intermission.to_le_bytes());
                self.data.extend(data.info.ref_params.paused.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.spectator.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.onground.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.waterlevel.to_le_bytes());
                self.data.extend(
                    data.info
                        .ref_params
                        .simvel
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(
                    data.info
                        .ref_params
                        .simorg
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(
                    data.info
                        .ref_params
                        .viewheight
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data
                    .extend(data.info.ref_params.idealpitch.to_le_bytes());
                self.data.extend(
                    data.info
                        .ref_params
                        .cl_viewangles
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data.extend(data.info.ref_params.health.to_le_bytes());
                self.data.extend(
                    data.info
                        .ref_params
                        .crosshairangle
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data
                    .extend(data.info.ref_params.viewsize.to_le_bytes());
                self.data.extend(
                    data.info
                        .ref_params
                        .punchangle
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data
                    .extend(data.info.ref_params.maxclients.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.viewentity.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.playernum.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.max_entities.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.demoplayback.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.hardware.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.smoothing.to_le_bytes());
                self.data.extend(data.info.ref_params.ptr_cmd.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.ptr_movevars.to_le_bytes());
                self.data.extend(
                    data.info
                        .ref_params
                        .viewport
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data
                    .extend(data.info.ref_params.next_view.to_le_bytes());
                self.data
                    .extend(data.info.ref_params.only_client_draw.to_le_bytes());
                // usercmd
                self.data.extend(data.info.usercmd.lerp_msec.to_le_bytes());
                self.data.extend(data.info.usercmd.msec.to_le_bytes());
                self.data.extend(0u8.to_le_bytes()); // unknown
                self.data.extend(
                    data.info
                        .usercmd
                        .viewangles
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                self.data
                    .extend(data.info.usercmd.forwardmove.to_le_bytes());
                self.data.extend(data.info.usercmd.sidemove.to_le_bytes());
                self.data.extend(data.info.usercmd.upmove.to_le_bytes());
                self.data.extend(data.info.usercmd.lightlevel.to_le_bytes());
                self.data.extend(0u8.to_le_bytes()); // unknown
                self.data.extend(data.info.usercmd.buttons.to_le_bytes());
                self.data.extend(data.info.usercmd.impulse.to_le_bytes());
                self.data
                    .extend(data.info.usercmd.weaponselect.to_le_bytes());
                self.data.extend(0u8.to_le_bytes()); // unknown
                self.data.extend(0u8.to_le_bytes()); // unknown
                self.data
                    .extend(data.info.usercmd.impact_index.to_le_bytes());
                self.data.extend(
                    data.info
                        .usercmd
                        .impact_position
                        .iter()
                        .map(|x| x.to_le_bytes())
                        .flatten(),
                );
                // movevars
                self.data.extend(data.info.movevars.gravity.to_le_bytes());
                self.data.extend(data.info.movevars.stopspeed.to_le_bytes());
                self.data.extend(data.info.movevars.maxspeed.to_le_bytes());
                self.data
                    .extend(data.info.movevars.spectatormaxspeed.to_le_bytes());
                self.data
                    .extend(data.info.movevars.accelerate.to_le_bytes());
                self.data
                    .extend(data.info.movevars.airaccelerate.to_le_bytes());
                self.data
                    .extend(data.info.movevars.wateraccelerate.to_le_bytes());
                self.data.extend(data.info.movevars.friction.to_le_bytes());
                self.data
                    .extend(data.info.movevars.edgefriction.to_le_bytes());
                self.data
                    .extend(data.info.movevars.waterfriction.to_le_bytes());
                self.data
                    .extend(data.info.movevars.entgravity.to_le_bytes());
                self.data.extend(data.info.movevars.bounce.to_le_bytes());
                self.data.extend(data.info.movevars.stepsize.to_le_bytes());
                self.data
                    .extend(data.info.movevars.maxvelocity.to_le_bytes());
                self.data.extend(data.info.movevars.zmax.to_le_bytes());
                self.data
                    .extend(data.info.movevars.wave_height.to_le_bytes());
                self.data.extend(data.info.movevars.footsteps.to_le_bytes());
                self.data.extend_from_slice(data.info.movevars.sky_name);
                self.data.extend(data.info.movevars.rollangle.to_le_bytes());
                self.data.extend(data.info.movevars.rollspeed.to_le_bytes());
                self.data
                    .extend(data.info.movevars.skycolor_r.to_le_bytes());
                self.data
                    .extend(data.info.movevars.skycolor_g.to_le_bytes());
                self.data
                    .extend(data.info.movevars.skycolor_b.to_le_bytes());
                self.data.extend(data.info.movevars.skyvec_x.to_le_bytes());
                self.data.extend(data.info.movevars.skyvec_y.to_le_bytes());
                self.data.extend(data.info.movevars.skyvec_z.to_le_bytes());
                // still in info
                self.data
                    .extend(data.info.view.iter().map(|x| x.to_le_bytes()).flatten());
                self.data.extend(data.info.viewmodel.to_le_bytes());
                // now other data
                self.data.extend(data.incoming_sequence.to_le_bytes());
                self.data.extend(data.incoming_acknowledged.to_le_bytes());
                self.data
                    .extend(data.incoming_reliable_acknowledged.to_le_bytes());
                self.data
                    .extend(data.incoming_reliable_sequence.to_le_bytes());
                self.data.extend(data.outgoing_sequence.to_le_bytes());
                self.data.extend(data.reliable_sequence.to_le_bytes());
                self.data.extend(data.last_reliable_sequence.to_le_bytes());
                self.data.extend((data.msg.len() as u32).to_le_bytes());
                self.data.extend_from_slice(data.msg);
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
        self.data.push(i);
        self.offset += 1;
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
}
