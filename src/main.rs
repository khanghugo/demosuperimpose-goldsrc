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
use types::EngineMessage;
use writer::DemoWriter;

use crate::demo_doer::netmsg_rewrite_test;

macro_rules! write_demo {
    ($demo_name:literal, $demo:ident) => {{
        let mut out = DemoWriter::new(String::from($demo_name));
        out.write_file($demo);
    }};
}

fn main() {
    let mut bytes = Vec::new();
    let mut f = File::open("./example/hl.dem").unwrap();
    f.read_to_end(&mut bytes);

    let mut demo = hldemo::Demo::parse(&bytes).unwrap();
    // example::example(&mut demo);
    // netmsg_rewrite_test::netmsg_rewrite_test(&mut demo);
    example::netmsg_parse_write_parse(&mut demo);

    // write_demo!("test.dem", demo);    




    // let mut bytes = Vec::new();
    // let mut f = File::open("./test.dem").unwrap();
    // f.read_to_end(&mut bytes);

    // let mut demo = hldemo::Demo::parse(&bytes).unwrap();
    // // example::example(&mut demo);
    // // netmsg_rewrite_test::netmsg_rewrite_test(&mut demo);
    // example::print_netmsg(&mut demo);

    // // write_demo!("test.dem", demo);  
}
