use std::collections::HashMap;

use demosuperimpose_goldsrc::netmsg_doer::{parse_netmsg, utils::get_initial_delta, write_netmsg};
use hldemo::{Demo, FrameData};

use super::*;

/// Simply parses netmsg.
pub fn netmsg_parse(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, netmsg) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();
            }
        }
    }
}

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

                data.msg = write.leak();
                // data.msg = &[]; // sanity check
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

    let mut i = 0;
    let mut j = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                println!("{} {}", i, j);
                let (_, messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                println!("{:?}", messages);

                let write = write_netmsg(messages, &delta_decoders, &custom_messages);

                let (_, parse_write) = parse_netmsg(
                    write.leak(),
                    &mut pw_delta_decoders,
                    &mut pw_custom_messages,
                )
                .unwrap();
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}

/// Simply parses netmsg.
pub fn example(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();
    let mut i = 0;
    let mut j = 0;

    // for entry in &mut demo.directory.entries {
    //     for frame in &mut entry.frames {
    //         // println!("{}", frame.time);
    //         if let FrameData::NetMsg((_, data)) = &mut frame.data {
    //             println!("{} {}", i, j);
    //             let (_, netmsg) =
    //                 parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

    //             if i == 0 && j == 3 {
    //                 // println!("{:#?}", netmsg);

    //                 println!("");
    //                 println!("fuck");
    //                 println!("");

    //                 let write = write_netmsg(netmsg, &delta_decoders, &custom_messages);
    //                 let (i, parse) =
    //                     parse_netmsg(&write, &mut delta_decoders, &mut custom_messages).unwrap();
    //                 // println!("{:#?}", parse);
    //                 // panic!();
    //             }

    //             for what in &netmsg {
    //                 if let Message::EngineMessage(EngineMessage::SvcPacketEntities(what)) = what {
    //                     // println!("{:#?}", what.entity_states);
    //                     // for entity in &what.entity_states {
    //                     //     if entity.entity_index == 1 {
    //                     //         println!("{:?}", entity);
    //                     //     }
    //                     // }
    //                 }

    //                 if let Message::EngineMessage(EngineMessage::SvcDeltaPacketEntities(what)) =
    //                     what
    //                 {
    //                     // println!("{:#?}", what.entity_states);
    //                     // for entity in &what.entity_states {
    //                     //     if entity.entity_index == 0 {
    //                     //         println!("{:?}", entity);
    //                     //     }
    //                     // }
    //                 }

    //                 if let Message::EngineMessage(EngineMessage::SvcSpawnBaseline(what)) = what {
    //                     for (index, entity) in what.entities.iter().enumerate() {
    //                         // println!("index {} entity {:?}", index, entity);
    //                     }
    //                 }
    //             }

    //             // // data.msg = again.leak();
    //             // data.msg = &[];
    //         }
    //         j += 1;
    //     }
    //     i += 1;
    //     j = 0;
    // }
}
