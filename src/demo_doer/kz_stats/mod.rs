use dem::{parse_netmsg, write_netmsg, Aux};

use crate::utils::Buttons;
use crate::wrap_message;

use self::add_keys::add_keys;
use self::add_speedometer::add_speedometer;

use super::*;

pub mod add_keys;
pub mod add_speedometer;

pub struct KzAddOns {
    keys: bool,
    speedometer: bool,
}

impl KzAddOns {
    pub fn new() -> Self {
        Self {
            keys: false,
            speedometer: false,
        }
    }

    pub fn add_keys(&mut self) -> &mut Self {
        self.keys = true;
        return self;
    }

    pub fn add_speedometer(&mut self) -> &mut Self {
        self.speedometer = true;
        return self;
    }

    pub fn get(&self) -> &Self {
        return self;
    }
}

#[derive(Debug)]
pub struct KzInfo<'a> {
    // First 3 members could only be found in netmessage.
    // Frame 0 0 is netmessage.
    // Frame 1 0 is not netmessage.
    forward: f32,
    side: f32,
    up: f32,
    origin: [f32; 3],
    viewangles: [f32; 3],
    // velocity: [f32; 3],
    buttons: u16,
    movetype: i32,
    weapon: i32,
    flags: u32,
    commands: &'a [u8],
    frametime: f32,
}

const VEC3EMPTY: [f32; 3] = [0., 0., 0.];

impl<'a> KzInfo<'a> {
    fn new(origin: [f32; 3], viewangles: [f32; 3], weapon: i32, frametime: f32) -> Self {
        Self {
            forward: 0.,
            side: 0.,
            up: 0.,
            origin,
            viewangles,
            // velocity: VEC3EMPTY,
            buttons: 0,
            movetype: 0,
            weapon,
            flags: 0,
            commands: &[],
            // accumulative
            frametime,
        }
    }
}

pub fn add_kz_stats(demo: &mut Demo, addons: &KzAddOns) {
    let mut aux = Aux::new();

    for (entry_idx, entry) in demo.directory.entries.iter_mut().enumerate() {
        let mut curr: Option<KzInfo> = None;
        let mut prev: Option<KzInfo> = None;

        for frame in &mut entry.frames {
            match &mut frame.data {
                FrameData::NetMsg((_, netmsg)) => {
                    let (_, mut messages) = parse_netmsg(netmsg.msg, &aux).unwrap();

                    if let Some(ref mut curr) = curr {
                        curr.forward = netmsg.info.usercmd.forwardmove;
                        curr.side = netmsg.info.usercmd.sidemove;
                        curr.up = netmsg.info.usercmd.upmove;
                        curr.buttons = netmsg.info.usercmd.buttons;
                        // movetype?
                        // weapon?
                        // flags?
                    }

                    if addons.speedometer {
                        if let Some(temp_entity) = add_speedometer(prev.as_ref(), curr.as_ref()) {
                            messages.push(wrap_message!(SvcTempEntity, temp_entity));
                        }
                    }

                    if addons.keys {
                        if let Some(temp_entity) = add_keys(curr.as_ref()) {
                            messages.push(wrap_message!(SvcTempEntity, temp_entity));
                        }
                    }

                    let write = write_netmsg(messages, &aux);
                    netmsg.msg = write.leak();
                }
                FrameData::ClientData(client_data) => {
                    prev = curr;
                    curr = Some(KzInfo::new(
                        client_data.origin,
                        client_data.viewangles,
                        client_data.weapon_bits,
                        frame.time,
                    ));
                }
                FrameData::ConsoleCommand(command) => {
                    if let Some(ref mut curr) = curr {
                        curr.commands = command.command;
                    }
                }
                _ => (),
            }
        }
    }
}

trait CoordConversion {
    fn coord_conversion(&self) -> i16;
}

impl CoordConversion for f32 {
    fn coord_conversion(&self) -> i16 {
        (self * 8192.).round() as i16
    }
}
