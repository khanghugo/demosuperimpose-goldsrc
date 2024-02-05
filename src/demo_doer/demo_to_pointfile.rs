use crate::open_demo;

use super::*;
use std::{fs::File, io::Write, path::Path};

pub fn demo_to_pointfile(demo: &Demo) {
    let ghost = get_ghost::demo::demo_ghost_parse("trenchbroom", demo, 0., false);
    let mut file = File::create("trenchbroom_player_point.txt").unwrap();

    for frame in ghost.frames {
        match write!(
            file,
            "{} {} {}\n",
            frame.origin[0], frame.origin[1], frame.origin[2]
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("Cannot write: {}", err)
            }
        }
    }
}

pub fn demo_to_pointfile_cli() {
    use std::env;

    let help = || {
        println!(
            "\
Output file is \"trenchbroom_player_point.txt\"

./binary <path to demo>"
        )
    };

    let wrap = |demo_file_name: &str| {
        let demo_file_name = Path::new(demo_file_name);
        let demo = open_demo!(demo_file_name);
        demo_to_pointfile(&demo);
    };

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => help(),
        2 => wrap(&args[1]),
        _ => help(),
    }
}
