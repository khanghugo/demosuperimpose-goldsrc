use std::{collections::HashMap, str::from_utf8};

use demosuperimpose_goldsrc::netmsg_doer::{
    parse_netmsg,
    utils::{get_initial_delta, BitReader},
    write_netmsg, NetMsgDoer,
};
use hldemo::{Demo, FrameData};

use super::*;

/// Simply parses netmsg.
pub fn example(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();
    let mut i = 0;
    let mut j = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                println!("{} {}", i, j);
                let (_, netmsg) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                // println!("parser: {:?}", netmsg);

                let l1 = netmsg.len();

                let again = write_netmsg(netmsg, &mut delta_decoders, &mut custom_messages);

                // let mut delta_decoders = get_initial_delta();
                // let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

                let (_, huh) =
                    parse_netmsg(again.leak(), &mut delta_decoders, &mut custom_messages).unwrap();

                let l2 = huh.len();

                assert!(l1 == l2);

                // println!("writer: {:?}", huh);
                // panic!()

                // data.msg = again.leak();
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
