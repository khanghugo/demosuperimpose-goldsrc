use std::collections::HashMap;

use demosuperimpose_goldsrc::netmsg_doer::{
    client_data, parse_netmsg,
    utils::{get_initial_delta, BitSliceCast},
    write_netmsg,
};
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

    let mut i = 0;
    let mut j = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                println!("{} {}", i, j);
                let (_, messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                let write = write_netmsg(messages, &delta_decoders, &custom_messages);

                data.msg = write.leak();
                // data.msg = &[]; // sanity check
            }
            j += 1;
        }
        i += 1;
        j = 0;
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

// for (a, b ) in what.iter().zip(huh) {
//     if a != b {
//         let totally = BitReader::new(&what);
//         println!("totally {:?}", totally.bytes);
//         let uhh = ClientData::parse(&what, &mut delta_decoders.clone()).unwrap().1;
//         let boo = ClientData::parse(&huh, &mut delta_decoders.clone()).unwrap().1;
//         println!("uhh {:?}", uhh);
//         println!("boo {:?}", boo);
//         println!("what {:?}", what);
//         println!("huh {:?}", huh);
//         println!("");
//         println!("");
//     }
// }

pub fn netmsg_parse_write_parse_extra(demo: &mut Demo) {
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

                // println!("{:?}", messages);

                let write = write_netmsg(messages, &delta_decoders, &custom_messages);

                let (_, parse_write) = parse_netmsg(
                    write.leak(),
                    &mut pw_delta_decoders,
                    &mut pw_custom_messages,
                )
                .unwrap();

                for what in &parse_write {
                    if let Message::EngineMessage(EngineMessage::SvcPacketEntities(what)) = what {
                        // println!("{:#?}", what);
                        // count += 1;
                        // println!("count {}", count);
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 1 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                    }

                    if let Message::EngineMessage(EngineMessage::SvcClientData(what)) = what {
                        // println!("{:?}", what);
                        // panic!()
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 0 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                        // count += 1;
                        // println!("count {}", count);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcStopSound(what)) = what {
                        // println!("{:?}", netmsg);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcStopSound(what)) = what {
                        // println!("{:?}", netmsg);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcSpawnBaseline(what)) = what {
                        for (index, entity) in what.entities.iter().enumerate() {
                            // println!("entity index {}", entity.index.to_u16());
                        }
                        // println!("{:#?}", what);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcTempEntity(what)) = what {
                        if what.entity_type == 29 {
                            // println!("{:?}", what.entity);
                        }
                    }

                    if let Message::EngineMessage(EngineMessage::SvcSpawnStatic(what)) = what {
                        // println!("{:#?}", what);
                    }
                }
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

    let mut count = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            // println!("frametime {}", frame.time);
            // println!("{}", frame.time);
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                println!("{} {}", i, j);

                // if j == 600 {
                //     panic!()
                // }
                let (_, netmsg) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();
                println!("{:#?}", netmsg);

                // println!("{} {} {}", i, j, netmsg.len());

                for what in &netmsg {
                    if let Message::EngineMessage(EngineMessage::SvcPacketEntities(what)) = what {
                        // println!("{:#?}", what);
                        // count += 1;
                        // println!("count {}", count);
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 1 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                    }

                    if let Message::EngineMessage(EngineMessage::SvcClientData(what)) = what {
                        // println!("{:?}", what);
                        // for entity in &what.entity_states {
                        //     if entity.entity_index == 0 {
                        //         println!("{:?}", entity);
                        //     }
                        // }
                        // count += 1;
                        // println!("count {}", count);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcStopSound(what)) = what {
                        // println!("{:?}", netmsg);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcStopSound(what)) = what {
                        // println!("{:?}", netmsg);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcSpawnBaseline(what)) = what {
                        for (index, entity) in what.entities.iter().enumerate() {
                            // println!("entity index {}", entity.index.to_u16());
                        }
                        // println!("{:#?}", what);
                    }

                    if let Message::EngineMessage(EngineMessage::SvcTempEntity(what)) = what {
                        if what.entity_type == 29 {
                            // println!("{:?}", what.entity);
                        }
                    }

                    if let Message::EngineMessage(EngineMessage::SvcSpawnStatic(what)) = what {
                        // println!("{:#?}", what);
                    }
                }

                // // data.msg = again.leak();
                // data.msg = &[];
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
