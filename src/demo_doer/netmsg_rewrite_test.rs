use std::collections::HashMap;

use demosuperimpose_goldsrc::netmsg_doer::{parse_netmsg, utils::get_initial_delta, write_netmsg};
use hldemo::{Demo, FrameData};

use super::*;

pub fn netmsg_rewrite_test(demo: &str) {
    let mut bytes = Vec::new();
    let mut f = File::open(demo).unwrap();
    f.read_to_end(&mut bytes).unwrap();

    let mut demo = hldemo::Demo::parse(bytes.leak()).unwrap(); // heh

    _netmsg_rewrite_test(&mut demo);

    write_demo!("test.dem", demo);
}

fn _netmsg_rewrite_test(demo: &mut Demo) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                let write = write_netmsg(messages, &mut delta_decoders, &custom_messages);

                data.msg = write.leak();
                // data.msg = &[];
            }
        }
    }
}
