extern crate hldemo;

use std::fs::File;
use std::io::Read;

mod demo_doer;
mod types;
mod writer;

use bitvec::bits;
use bitvec::prelude::*;
use demo_doer::example;
use demosuperimpose_goldsrc::writer::BitWriter;
use hldemo::{FrameData, NetMsgData};
use types::EngineMessageType;
use writer::DemoWriter;

fn print_u8_array(i: &[u8]) {
    i.iter().for_each(|x| print!("{}", *x as char));
    println!("");
}

fn main() {
    let mut bytes = Vec::new();
    let mut f = File::open("./example/gold.dem").unwrap();
    f.read_to_end(&mut bytes);

    let mut demo = hldemo::Demo::parse(&bytes).unwrap();
    example::example(&mut demo);

    // let mut what = DemoWriter::new(String::from("out_example.dem"));
    // what.write_file(demo);

    // let mut what = BitWriter::new();
    // for _ in 0..63 {
    //     what.append_bit(true);
    // }
    // what.append_slice(&[true, true]);
    // // let huh = what.get_u8_slice();
    // println!("{:?}", what.get_u8_vec());

    // let immut = bits![u8, Lsb0; 0, 1, 0, 0, 1, 0, 0, 1];
    // println!("{}", immut);

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
