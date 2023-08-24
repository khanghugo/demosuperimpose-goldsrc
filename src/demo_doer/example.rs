use std::{collections::HashMap, str::from_utf8};

use bincode::de;
use demosuperimpose_goldsrc::netmsg_doer::{
    parse_netmsg,
    utils::{get_initial_delta, BitReader},
    write_netmsg, NetMsgDoer,
};
use hldemo::{Demo, FrameData, parse::frame};

use super::*;

/// Simply prints netmsg.
pub fn print_netmsg(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();
    let mut i = 0;
    let mut j = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, netmsg) =
                parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                println!("{} {} {:?}", i, j, netmsg);
            }

            j += 1;
        }

        i += 1;
        j = 0;
    }
}

/// Simply parse-write netmsg.
pub fn netmsg_parse_write(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                let write = write_netmsg(messages, &delta_decoders, &custom_messages);

                // println!("parsed {:?}", data.msg);
                // println!("writed {:?}", write);

                data.msg = write.leak();
            }
        }
    }
}

/// Simply parse-write-parse netmsg.
pub fn netmsg_parse_write_parse(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    let mut pw_delta_decoders = get_initial_delta();
    let mut pw_custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {

                println!("");
                println!("New frame");
                println!("");
                println!("");
                println!("");
                println!("");
                let (_, messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                println!("");
                println!("Write part");
                println!("");

                let write = write_netmsg(messages, &delta_decoders, &custom_messages);
                let (_, parse_write) = parse_netmsg(write.leak(), &mut pw_delta_decoders, &mut pw_custom_messages).unwrap();
            }
        }
    }
}

/// Simply parses netmsg.
pub fn example(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();
    let mut i = 0;
    let mut j = 0;

    // let huh = &demo.directory.entries[0].frames[0];
    // if let FrameData::NetMsg((_, data)) = &huh.data {
    //     println!("{:?}", data.msg);
    // }

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            // println!("{}", frame.time);
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                // println!("{} {}", i, j);
                let (_, netmsg) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                for what in &netmsg {
                    if let Message::EngineMessage(EngineMessage::SvcPacketEntities(what)) = what {
                        // println!("{:#?}", what.entity_states);
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 1 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                    }

                    if let Message::EngineMessage(EngineMessage::SvcDeltaPacketEntities(what)) =
                        what
                    {
                        // println!("{:#?}", what.entity_states);
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 0 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                    }

                    if let Message::EngineMessage(EngineMessage::SvcSpawnBaseline(what)) = what {
                        // println!("{:#?}", what);
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 0 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                    }
                }

                // println!("parser: {:?}", netmsg);

                // let l1 = netmsg.len();

                // let again = write_netmsg(netmsg, &mut delta_decoders, &mut custom_messages);

                // // let mut delta_decoders = get_initial_delta();
                // // let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

                // // let (_, huh) =
                // //     parse_netmsg(again.leak(), &mut delta_decoders, &mut custom_messages).unwrap();

                // // let l2 = huh.len();

                // // assert!(l1 == l2);

                // // println!("writer: {:?}", huh);
                // // panic!()

                // // data.msg = again.leak();
                // data.msg = &[];
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
