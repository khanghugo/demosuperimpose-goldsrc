extern crate hldemo;

use std::fs::File;
use std::io::Read;

use demo_doer::{
    add_debug, example,
    kz_stats::{self, add_kz_stats, KzAddOns},
    netmsg_rewrite_test,
    superimpose::{self, superimpose},
};

use crate::demo_doer::{offset_viewangles, remove_entities, superimpose::superimpose_folder};

mod demo_doer;
mod types;
mod utils;
mod writer;

fn main() {
    // let mut demo = open_demo!("example/long.dem");
    let demo = superimpose::superimpose("example/gold.dem".to_string(), vec![
        ("example/gold2.dem".to_owned(),  0.),
        ("example/gold3.dem".to_owned(), 0.),
        ("example/gold4.dem".to_owned(), 0.),
        ]);
    // kz_stats::add_kz_stats(
    //     &mut demo,
    //     KzAddOns::new().add_speedometer().add_keys().get(),
    // );
    // offset_viewangles::front_flip(&mut demo, 230, 350);
    // offset_viewangles::back_flip(&mut demo, 500, 750);
    // add_debug::add_debug(&mut demo);
    // example::print_netmsg(&mut demo);
    // offset_viewangles::back_flip(&mut demo, 230, 350);
    write_demo!("out.dem", demo);
}
