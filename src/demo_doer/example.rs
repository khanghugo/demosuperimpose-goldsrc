use std::{collections::HashMap, str::from_utf8};

use demosuperimpose_goldsrc::netmsg_doer::{
    client_data::ClientData,
    delta_description::DeltaDescription,
    parse_netmsg,
    print::Print,
    send_extra_info::SendExtraInfo,
    server_info::{self, ServerInfo},
    time::Time,
    utils::{get_initial_delta, BitReader},
    NetMsgDoer,
};
use hldemo::{Demo, FrameData};

use super::*;

/// Simply parses netmsg.
pub fn example(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for (i, entry) in demo.directory.entries.iter().enumerate() {
        for (j, frame) in entry.frames.iter().enumerate() {
            if let FrameData::NetMsg((_, data)) = &frame.data {
                println!("{} {}", i, j);
                parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages);
            }
        }
    }
}
