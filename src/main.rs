extern crate hldemo;

use std::fs::File;
use std::io::Read;

use demo_doer::{example, netmsg_rewrite_test, superimpose};

use crate::demo_doer::{add_speedometer, remove_entities};

mod demo_doer;
mod types;
mod utils;
mod writer;

fn main() {
    // let mut bytes = Vec::new();
    // let mut f = File::open("./example/hldm.dem").unwrap();
    // // let mut f = File::open("./test.dem").unwrap();

    // f.read_to_end(&mut bytes).unwrap();

    // let mut demo = hldemo::Demo::parse(&bytes).unwrap();
    // example::example(&mut demo);
    // // example::netmsg_parse_write_parse_extra(&mut demo);
    // // netmsg_rewrite_test::netmsg_rewrite_test(&mut demo);
    // // example::netmsg_parse(&mut demo);
    // // example::netmsg_parse_write_parse(&mut demo);
    // example::netmsg_parse_write(&mut demo);
    // // add_speedometer::add_speed_o_meter(&mut demo);
    // write_demo!("test.dem", demo);

    superimpose::superimpose(
        "./example/gold.dem".to_string(),
        vec![
            ("./example/gold2.dem".to_string(), 0.),
            // ("./example/gold3.dem".to_string(), 0.),
            // ("./example/gold4.dem".to_string(), 0.),
            // ("./example/gold5.dem".to_string(), 0.),
        ],
    );

    // netmsg_rewrite_test::netmsg_rewrite_test("./example/gold.dem");

    // remove_entities::remove_entities(&mut demo, vec![68u16]);
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
