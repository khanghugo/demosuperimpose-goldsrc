use dem::{
    parse_netmsg,
    types::{EngineMessage, NetMessage},
    write_netmsg, Aux,
};

use super::*;

#[allow(dead_code)]
/// Simply parses netmsg.
pub fn netmsg_parse(demo: &mut Demo) {
    let aux: Aux = Aux::new();

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, _netmsg) = parse_netmsg(data.msg, aux.clone()).unwrap();
            }
        }
    }
}

#[allow(dead_code)]
/// Simply prints netmsg.
pub fn print_netmsg(demo: &mut Demo) {
    let aux: Aux = Aux::new();

    let mut i = 0;
    let mut j = 0;

    for (entry_idx, entry) in demo.directory.entries.iter_mut().enumerate() {
        for (frame_idx, frame) in entry.frames.iter_mut().enumerate() {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, netmsg) = parse_netmsg(data.msg, aux.clone()).unwrap();

                for what in &netmsg {
                    match what {
                        NetMessage::EngineMessage(huh) => match **huh {
                            EngineMessage::SvcNewUserMsg(_) => {
                                println!("1 {} {}", entry_idx, frame_idx)
                            }
                            EngineMessage::SvcDeltaDescription(_) => {
                                println!("2 {} {}", entry_idx, frame_idx)
                            }
                            EngineMessage::SvcServerInfo(_) => {
                                println!("3 {} {}", entry_idx, frame_idx)
                            }

                            _ => (),
                        },
                        _ => (),
                    };
                }

                println!("{} {} {:?}", i, j, netmsg);
            }

            j += 1;
        }

        i += 1;
        j = 0;
    }
}

#[allow(dead_code)]
/// Simply parse-write netmsg.
pub fn netmsg_parse_write(demo: &mut Demo) {
    let aux: Aux = Aux::new();

    let mut i = 0;
    let mut j = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                println!("{} {}", i, j);
                let (_, messages) = parse_netmsg(data.msg, aux.clone()).unwrap();

                let write = write_netmsg(messages, aux.clone());

                data.msg = write.leak();
                // data.msg = &[]; // sanity check
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}

#[allow(dead_code)]
/// Simply parse-write-parse netmsg.
pub fn netmsg_parse_write_parse(demo: &mut Demo) {
    let aux: Aux = Aux::new();

    let aux2: Aux = Aux::new();

    let mut i = 0;
    let mut j = 0;

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                println!("{} {}", i, j);
                let (_, messages) = parse_netmsg(data.msg, aux.clone()).unwrap();

                println!("{:?}", messages);

                let write = write_netmsg(messages, aux.clone());

                let (_, _parse_write) = parse_netmsg(write.leak(), aux2.clone()).unwrap();
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
