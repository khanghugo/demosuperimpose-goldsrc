use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use hldemo::{Demo, Directory, DirectoryEntry, Frame, FrameData, Header};
use std::fs;
use std::io::Write;

use bitvec::prelude::Lsb0;

pub struct ByteWriter {
    pub data: Vec<u8>,
    // Offset isn't really needed because we do vector and we can find offset easily.
    // But all in the spirit of 🚀.
    offset: usize,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
        }
    }

    fn offset(&mut self, offset: usize) {
        self.offset += offset;
    }

    pub fn append_u32(&mut self, i: u32) {
        self.data.extend(i.to_le_bytes());
        self.offset(4);
    }

    pub fn append_i32(&mut self, i: i32) {
        self.data.extend(i.to_le_bytes());
        self.offset(4);
    }

    pub fn append_f32(&mut self, i: f32) {
        self.data.extend(i.to_le_bytes());
        self.offset(4);
    }

    pub fn append_u8(&mut self, i: u8) {
        self.data.extend(i.to_le_bytes());
        self.offset(1);
    }

    pub fn append_i8(&mut self, i: i8) {
        self.data.extend(i.to_le_bytes());
        self.offset(1);
    }

    pub fn append_u16(&mut self, i: u16) {
        self.data.extend(i.to_le_bytes());
        self.offset(2);
    }

    pub fn append_i16(&mut self, i: i16) {
        self.data.extend(i.to_le_bytes());
        self.offset(2);
    }

    pub fn append_u8_slice(&mut self, i: &[u8]) {
        self.data.extend_from_slice(i);
        self.offset(i.len());
    }

    pub fn append_f32_array(&mut self, i: [f32; 3]) {
        self.data
            .extend(i.iter().map(|x| x.to_le_bytes()).flatten());
        self.offset(4 * 3);
    }

    pub fn append_i32_array_4(&mut self, i: [i32; 4]) {
        self.data
            .extend(i.iter().map(|x| x.to_le_bytes()).flatten());
        self.offset(4 * 4);
    }
}

#[derive(Debug)]
pub struct BitWriter {
    pub data: BitVec<u8, bitvec::order::Lsb0>,
    pub offset: usize,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            data: BitVec::new(),
            offset: 0,
        }
    }

    fn offset(&mut self, i: usize) {
        self.offset += i;
    }

    pub fn append_bit(&mut self, i: bool) {
        self.data.push(i);
        self.offset(1);
    }

    pub fn append_slice(&mut self, i: &BitSlice<u8>) {
        self.data.extend(i);
        self.offset(i.len());
    }

    pub fn append_vec(&mut self, i: BitVec<u8>) {
        self.append_slice(i.as_bitslice())
    }

    pub fn append_u8(&mut self, i: u8) {
        let bits: BitVec<u8> = BitVec::<u8, Lsb0>::from_element(i);
        self.append_vec(bits);
    }

    /// Append selected bits from a u32.
    /// end = 31 means excluding the sign bit due to LE.
    pub fn append_u32_range(&mut self, i: u32, end: u32) {
        let bits: BitVec<u8> = i
            .to_le_bytes()
            .iter()
            .map(|byte| BitVec::<u8, Lsb0>::from_element(*byte))
            .flatten()
            .collect();

        self.append_slice(&bits[..end as usize]);
    }

    pub fn insert_bit(&mut self, i: bool, pos: usize) {
        self.data.insert(pos, i);
        self.offset(1);
    }

    pub fn insert_slice(&mut self, i: &BitSlice<u8>, pos: usize) {
        for (offset, what) in i.iter().enumerate() {
            self.insert_bit(*what, pos + offset);
        }
    }

    pub fn insert_vec(&mut self, i: BitVec<u8>, pos: usize) {
        self.insert_slice(i.as_bitslice(), pos);
    }

    pub fn insert_u8(&mut self, i: u8, pos: usize) {
        let bits: BitVec<u8> = BitVec::<u8, Lsb0>::from_element(i);
        self.insert_slice(&bits, pos);
    }

    pub fn insert_u32_range(&mut self, i: u32, end: u32, pos: usize) {
        let bits: BitVec<u8> = i
            .to_le_bytes()
            .iter()
            .map(|byte| BitVec::<u8, Lsb0>::from_element(*byte))
            .flatten()
            .collect();

        self.insert_slice(&bits[..end as usize], pos);
    }

    pub fn get_u8_vec(&mut self) -> Vec<u8> {
        // https://github.com/ferrilab/bitvec/issues/27
        let mut what = self.data.to_owned();
        what.force_align();
        what.into_vec()
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }
}

pub struct DemoWriter {
    pub filename: String,
    writer: ByteWriter,
}

impl DemoWriter {
    pub fn new(filename: String) -> DemoWriter {
        DemoWriter {
            filename,
            writer: ByteWriter::new(),
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

        let _ = file.write_all(&self.writer.data);
    }

    fn write_demo(&mut self, demo: Demo) {
        // Magic has 8 bytes in total
        self.writer.append_u8_slice("HLDEMO\x00\x00".as_bytes());

        self.write_header(demo.header);
        self.write_directory(demo.directory);
    }

    fn write_header(&mut self, header: Header) {
        self.writer.append_i32(header.demo_protocol);
        self.writer.append_i32(header.net_protocol);
        self.writer.append_u8_slice(header.map_name);
        self.writer.append_u8_slice(header.game_dir);
        self.writer.append_u32(header.map_crc);
    }

    fn write_directory(&mut self, directory: Directory) {
        let directory_offset_pos = self.writer.offset;
        let mut entry_offsets: Vec<usize> = Vec::new();

        // Delay writing directory offset
        self.writer.append_i32(0i32);

        for entry in &directory.entries {
            let mut has_written_next_section = false;

            entry_offsets.push(self.writer.offset);

            for frame in &entry.frames {
                self.write_frame(&frame);

                if matches!(frame.data, FrameData::NextSection) {
                    has_written_next_section = true;
                }
            }

            if !has_written_next_section {
                self.writer.append_u8(5u8);
                self.writer.append_f32(0.);
                self.writer.append_i32(0);
            }
        }

        let director_offset = self.writer.offset;
        self.writer.append_i32(directory.entries.len() as i32);

        for (entry, offset) in directory.entries.iter().zip(entry_offsets.iter()) {
            self.write_directory_entry(entry, offset);
        }

        self.writer.data.splice(
            directory_offset_pos..directory_offset_pos + 4,
            (director_offset as u32).to_le_bytes(),
        );
    }

    fn write_directory_entry(&mut self, entry: &DirectoryEntry, new_offset: &usize) {
        self.writer.append_i32(entry.entry_type);
        self.writer.append_u8_slice(entry.description);
        self.writer.append_i32(entry.flags);
        self.writer.append_i32(entry.cd_track);
        self.writer.append_f32(entry.track_time);
        self.writer.append_i32(entry.frame_count);
        self.writer.append_i32(*new_offset as i32);
        self.writer.append_i32(entry.file_length);
    }

    fn write_frame(&mut self, frame: &Frame) {
        match frame.data {
            FrameData::DemoStart => self.writer.append_u8(2u8),
            FrameData::ConsoleCommand(_) => self.writer.append_u8(3u8),
            FrameData::ClientData(_) => self.writer.append_u8(4u8),
            FrameData::NextSection => self.writer.append_u8(5u8),
            FrameData::Event(_) => self.writer.append_u8(6u8),
            FrameData::WeaponAnim(_) => self.writer.append_u8(7u8),
            FrameData::Sound(_) => self.writer.append_u8(8u8),
            FrameData::DemoBuffer(_) => self.writer.append_u8(9u8),
            FrameData::NetMsg(_) => self.writer.append_u8(0u8),
        }

        self.writer.append_f32(frame.time);
        self.writer.append_i32(frame.frame);
        self.write_frame_data(&frame.data);
    }
    fn write_frame_data(&mut self, frame: &FrameData) {
        match frame {
            FrameData::DemoStart => (),
            FrameData::ConsoleCommand(frame) => self.writer.append_u8_slice(frame.command),
            FrameData::ClientData(frame) => {
                self.writer.append_f32_array(frame.origin);
                self.writer.append_f32_array(frame.viewangles);
                self.writer.append_i32(frame.weapon_bits);
                self.writer.append_f32(frame.fov);
            }
            FrameData::NextSection => (),
            FrameData::Event(frame) => {
                self.writer.append_i32(frame.flags);
                self.writer.append_i32(frame.index);
                self.writer.append_f32(frame.delay);

                self.writer.append_i32(frame.args.flags);
                self.writer.append_i32(frame.args.entity_index);
                self.writer.append_f32_array(frame.args.origin);
                self.writer.append_f32_array(frame.args.angles);
                self.writer.append_f32_array(frame.args.velocity);
                self.writer.append_i32(frame.args.ducking);
                self.writer.append_f32(frame.args.fparam1);
                self.writer.append_f32(frame.args.fparam2);
                self.writer.append_i32(frame.args.iparam1);
                self.writer.append_i32(frame.args.iparam2);
                self.writer.append_i32(frame.args.bparam1);
                self.writer.append_i32(frame.args.bparam2);
            }
            FrameData::WeaponAnim(frame) => {
                self.writer.append_i32(frame.anim);
                self.writer.append_i32(frame.body);
            }
            FrameData::Sound(frame) => {
                self.writer.append_i32(frame.channel);
                self.writer.append_i32(frame.sample.len() as i32);
                self.writer.append_u8_slice(frame.sample);
                self.writer.append_f32(frame.attenuation);
                self.writer.append_f32(frame.volume);
                self.writer.append_i32(frame.flags);
                self.writer.append_i32(frame.pitch);
            }
            FrameData::DemoBuffer(frame) => {
                self.writer.append_i32(frame.buffer.len() as i32);
                self.writer.append_u8_slice(frame.buffer);
            }
            FrameData::NetMsg((_type_, data)) => {
                self.writer.append_f32(data.info.timestamp);
                // ref_params
                self.writer.append_f32_array(data.info.ref_params.vieworg);
                self.writer
                    .append_f32_array(data.info.ref_params.viewangles);
                self.writer.append_f32_array(data.info.ref_params.forward);
                self.writer.append_f32_array(data.info.ref_params.right);
                self.writer.append_f32_array(data.info.ref_params.up);
                self.writer.append_f32(data.info.ref_params.frametime);
                self.writer.append_f32(data.info.ref_params.time);
                self.writer.append_i32(data.info.ref_params.intermission);
                self.writer.append_i32(data.info.ref_params.paused);
                self.writer.append_i32(data.info.ref_params.spectator);
                self.writer.append_i32(data.info.ref_params.onground);
                self.writer.append_i32(data.info.ref_params.waterlevel);
                self.writer.append_f32_array(data.info.ref_params.simvel);
                self.writer.append_f32_array(data.info.ref_params.simorg);
                self.writer
                    .append_f32_array(data.info.ref_params.viewheight);
                self.writer.append_f32(data.info.ref_params.idealpitch);
                self.writer
                    .append_f32_array(data.info.ref_params.cl_viewangles);
                self.writer.append_i32(data.info.ref_params.health);
                self.writer
                    .append_f32_array(data.info.ref_params.crosshairangle);
                self.writer.append_f32(data.info.ref_params.viewsize);
                self.writer
                    .append_f32_array(data.info.ref_params.punchangle);
                self.writer.append_i32(data.info.ref_params.maxclients);
                self.writer.append_i32(data.info.ref_params.viewentity);
                self.writer.append_i32(data.info.ref_params.playernum);
                self.writer.append_i32(data.info.ref_params.max_entities);
                self.writer.append_i32(data.info.ref_params.demoplayback);
                self.writer.append_i32(data.info.ref_params.hardware);
                self.writer.append_i32(data.info.ref_params.smoothing);
                self.writer.append_i32(data.info.ref_params.ptr_cmd);
                self.writer.append_i32(data.info.ref_params.ptr_movevars);
                self.writer
                    .append_i32_array_4(data.info.ref_params.viewport);
                self.writer.append_i32(data.info.ref_params.next_view);
                self.writer
                    .append_i32(data.info.ref_params.only_client_draw);
                // usercmd
                self.writer.append_i16(data.info.usercmd.lerp_msec);
                self.writer.append_u8(data.info.usercmd.msec);
                self.writer.append_u8(0u8); // unknown
                self.writer.append_f32_array(data.info.usercmd.viewangles);
                self.writer.append_f32(data.info.usercmd.forwardmove);
                self.writer.append_f32(data.info.usercmd.sidemove);
                self.writer.append_f32(data.info.usercmd.upmove);
                self.writer.append_i8(data.info.usercmd.lightlevel);
                self.writer.append_u8(0u8); // unknown
                self.writer.append_u16(data.info.usercmd.buttons);
                self.writer.append_i8(data.info.usercmd.impulse);
                self.writer.append_i8(data.info.usercmd.weaponselect);
                self.writer.append_u8(0u8); // unknown
                self.writer.append_u8(0u8); // unknown
                self.writer.append_i32(data.info.usercmd.impact_index);
                self.writer
                    .append_f32_array(data.info.usercmd.impact_position);
                // movevars
                self.writer.append_f32(data.info.movevars.gravity);
                self.writer.append_f32(data.info.movevars.stopspeed);
                self.writer.append_f32(data.info.movevars.maxspeed);
                self.writer.append_f32(data.info.movevars.spectatormaxspeed);
                self.writer.append_f32(data.info.movevars.accelerate);
                self.writer.append_f32(data.info.movevars.airaccelerate);
                self.writer.append_f32(data.info.movevars.wateraccelerate);
                self.writer.append_f32(data.info.movevars.friction);
                self.writer.append_f32(data.info.movevars.edgefriction);
                self.writer.append_f32(data.info.movevars.waterfriction);
                self.writer.append_f32(data.info.movevars.entgravity);
                self.writer.append_f32(data.info.movevars.bounce);
                self.writer.append_f32(data.info.movevars.stepsize);
                self.writer.append_f32(data.info.movevars.maxvelocity);
                self.writer.append_f32(data.info.movevars.zmax);
                self.writer.append_f32(data.info.movevars.wave_height);
                self.writer.append_i32(data.info.movevars.footsteps);
                self.writer.append_u8_slice(data.info.movevars.sky_name);
                self.writer.append_f32(data.info.movevars.rollangle);
                self.writer.append_f32(data.info.movevars.rollspeed);
                self.writer.append_f32(data.info.movevars.skycolor_r);
                self.writer.append_f32(data.info.movevars.skycolor_g);
                self.writer.append_f32(data.info.movevars.skycolor_b);
                self.writer.append_f32(data.info.movevars.skyvec_x);
                self.writer.append_f32(data.info.movevars.skyvec_y);
                self.writer.append_f32(data.info.movevars.skyvec_z);
                // still in info
                self.writer.append_f32_array(data.info.view);
                self.writer.append_i32(data.info.viewmodel);
                // now other data
                self.writer.append_i32(data.incoming_sequence);
                self.writer.append_i32(data.incoming_acknowledged);
                self.writer.append_i32(data.incoming_reliable_acknowledged);
                self.writer.append_i32(data.incoming_reliable_sequence);
                self.writer.append_i32(data.outgoing_sequence);
                self.writer.append_i32(data.reliable_sequence);
                self.writer.append_i32(data.last_reliable_sequence);

                self.writer.append_i32(data.msg.len() as i32);
                self.writer.append_u8_slice(data.msg);
            }
        }
    }
}
