extern crate hldemo;

use std::fs::File;
use std::io::Read;

use demo_doer::{
    add_debug, example, netmsg_rewrite_test,
    remove_te_textmessage::remove_te_textmesasge,
    superimpose::{self, superimpose},
};

use crate::demo_doer::{add_speedometer, remove_entities, superimpose::superimpose_folder};

mod demo_doer;
mod types;
mod utils;
mod writer;

fn main() {
    let mut demo = open_demo!("whatdemo.dem");
    add_speedometer::add_speed_o_meter(&mut demo);
    write_demo!("out.dem", demo)
}
