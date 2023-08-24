extern crate hldemo;

use std::fs::File;
use std::io::Read;

mod demo_doer;
mod types;
mod utils;
mod writer;

fn main() {
    let mut bytes = Vec::new();
    let mut f = File::open("./example/gold.dem").unwrap();
    f.read_to_end(&mut bytes);

    let mut demo = hldemo::Demo::parse(&bytes).unwrap();
    // example::example(&mut demo);
    // netmsg_rewrite_test::netmsg_rewrite_test(&mut demo);
    // example::netmsg_parse(&mut demo);
    // example::netmsg_parse_write_parse(&mut demo);
    // example::netmsg_parse_write(&mut demo);
    write_demo!("test.dem", demo);

    // let mut bytes = Vec::new();
    // let mut f = File::open("./test.dem").unwrap();
    // f.read_to_end(&mut bytes);

    // let mut demo = hldemo::Demo::parse(&bytes).unwrap();
    // // example::example(&mut demo);
    // // netmsg_rewrite_test::netmsg_rewrite_test(&mut demo);
    // example::print_netmsg(&mut demo);

    // // write_demo!("test.dem", demo);
}
