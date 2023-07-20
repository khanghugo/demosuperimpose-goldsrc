extern crate hldemo;

use std::fs::File;
use std::io::Read;

mod types;
mod writer;

use hldemo::{FrameData, NetMsgData};
use nom::{
    bytes::complete::{tag, take_until, take_until1},
    character::complete::char,
    combinator::map,
    multi::count,
    number::complete::{le_i32, le_u8},
    sequence::{terminated, tuple},
    AsChar, IResult,
};
use types::{EngineMessageType, ServerInfo};
use writer::DemoWriter;

fn parse_server_info(i: &[u8]) -> IResult<&[u8], ServerInfo> {
    map(
        tuple((
            le_i32,
            le_i32,
            le_i32,
            count(le_u8, 16),
            le_u8,
            le_u8,
            le_u8,
            terminated(take_until("\x00"), tag("\x00")),
            terminated(take_until("\x00"), tag("\x00")),
            terminated(take_until("\x00"), tag("\x00")),
            terminated(take_until("\x00"), tag("\x00")),
            le_u8,
        )),
        |(
            protocol,
            spawn_count,
            map_checksum,
            client_dll_hash,
            max_players,
            player_index,
            is_deathmatch,
            game_dir,
            hostname,
            map_file_name,
            map_cycle,
            unknown,
        )| ServerInfo {
            protocol,
            spawn_count,
            map_checksum,
            client_dll_hash,
            max_players,
            player_index,
            is_deathmatch,
            game_dir,
            hostname,
            map_file_name,
            map_cycle,
            unknown,
        },
    )(i)
}

fn print_u8_array(i: &[u8]) {
    i.iter().for_each(|x| print!("{}", *x as char));
    println!("");
}

fn main() {
    let mut bytes = Vec::new();
    let mut f = File::open("./example/gold.dem").unwrap();
    f.read_to_end(&mut bytes);

    let demo = hldemo::Demo::parse(&bytes).unwrap();
    let mut what = DemoWriter::new(String::from("out_gold.dem"));
    what.write_file(demo);

    // println!("{:?}", demo.directory.entries[1].frame_count);
    // println!("{:?}", demo.directory.entries[1].frames.len());
    // for i in 0..demo.directory.entries[1].frames.len() as usize {
    //     // println!("{}", demo.directory.entries[1].frames[i].frame.i)
    //     if let FrameData::NetMsg(data) = &demo.directory.entries[1].frames[i].data {
    //         // println!("{:?}", data.info.ref_params.cl_viewangles);
    //         match data.msg[0] {
    //             5 => {
    //                 // SVC_SETVIEW
    //                 println!("SVC_SETVIEW")
    //             }
    //             11 => {
    //                 // SVC_SERVERINFO
    //                 // println!("this runs");

    //                 match parse_server_info(&data.msg[1..]) {
    //                     Ok((_, msg)) => {
    //                         print_u8_array(msg.map_cycle);
    //                     }
    //                     _ => (),
    //                 }
    //             }
    //             22 => {
    //                 // SVC_SPAWNBASELINE
    //                 println!("SVC_SPAWNBASELINE");
    //             }
    //             40 => {
    //                 // SVC_PACKETENTITIES
    //                 println!("SVC_PACKETENTITIES");
    //             }
    //             _ => (),
    //         }
    //         // let (_, data2) = parse_server_info(data.msg);

    //         // println!("{:?}", data.msg);
    //         // match parse_server_info(&data.msg[1..]) {
    //         //     Ok((_, msg)) => {
    //         //         print_u8_array(msg.map_cycle);
    //         //     },
    //         //     _ => (),
    //         // }
    //     }
    // }

    // demo.header.map_name.iter().for_each(|x| print!("{}", *x as char));

    // println!("what");
    // for i in demo.header.map_name {
    //     print!("{}", *i as char);
    // }
    // println!("fuck");

    // println!("{:?}", demo.header.map_name);
}
