use demosuperimpose_goldsrc::netmsg_doer::{server_info::ServerInfo, NetMsgDoer};
use hldemo::{Demo, FrameData};

use super::*;

pub fn example(demo: &mut Demo) {
    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                // ServerInfo::parse(data.msg);
                // Parse first byte to know type
                // if matches!(parse_netmsg_message_type(data.msg[0]), NetMsgMessageType::EngineMessage(EngineMessageType::SvcServerInfo)) {
                //     println!("this happens");
                //     // Skip 1 byte because we already know type
                //     if let Ok((_, mut server_info)) = parse_server_info(&data.msg[1..]) {
                //         println!("before {:?}", data.msg);
                //         // Null terminator appended in writer instead
                //         server_info.game_dir = "cstrike".as_bytes();
                //         let (bytes, _) = write_server_info(server_info);
                //         let bytes = bytes.leak();
                //         data.msg = bytes;
                //         println!("after {:?}", data.msg);
                //     }
                // }
            }
        }
    }
}
