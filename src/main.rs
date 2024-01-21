extern crate hldemo;

use std::{fs::File, path::Path};
use std::io::Read;

use demo_doer::{
    add_debug, example, ghost_to_demo,
    kz_stats::{self, add_kz_stats, KzAddOns},
    netmsg_rewrite_test,
    superimpose::{self, superimpose},
};

mod demo_doer;
mod types;
mod utils;
mod writer;

fn main() {
    // let mut demo = open_demo!("example/rvp.dem");
    // let mut demo = open_demo!("out.dem");
    // example::print_netmsg(&mut demo);
    // example::change_map_checksum(&mut demo);
    // example::rawe(&mut demo);
    // example::remove_netmsg_info(&mut demo);
    // example::remove_removable_messages(&mut demo);
    let mut demo = ghost_to_demo::ghost_to_demo(
        Path::new("example/rvp.rj.json"),
        Path::new("example/rvp_tundra-bhop.bsp"),
    );
    // let demo = superimpose::superimpose(
    //     "example/gold.dem".to_string(),
    //     vec![
    //         ("example/gold2.dem".to_owned(), 0.),
    //         ("example/gold3.dem".to_owned(), 0.),
    //         ("example/gold4.dem".to_owned(), 0.),
    //     ],
    // );
    // kz_stats::add_kz_stats(
    //     &mut demo,
    //     KzAddOns::new().add_speedometer().add_keys().get(),
    // );
    // offset_viewangles::front_flip(&mut demo, 230, 350);
    // offset_viewangles::back_flip(&mut demo, 500, 750);
    // add_debug::add_debug(&mut demo);
    // example::print_netmsg(&mut demo);
    // offset_viewangles::back_flip(&mut demo, 230, 350);
    // ghost_to_demo::bsp::get_map_info("example/rvp_tundra-bhop.bsp");
    // example::netmsg_parse_write_parse(&mut demo);
    write_demo!("out.dem", demo);
}
