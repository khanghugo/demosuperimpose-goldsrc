use crate::open_demo;

use super::*;
use std::{fs::File, io::Write, path::Path};


/// Generates a pointfile (.pts) to visualize the player's path within the level editor
/// The pointfile can be loaded into JACK or Trenchbroom
pub fn demo_to_pointfile(demo: &Demo, output_name: &str) {
    let ghost = get_ghost::demo::demo_ghost_parse("player_path", demo, 0., false);
    let mut pointfile = File::create(format!("{}.pts", output_name)).unwrap();

    for frame in ghost.frames {
        match write!(
            pointfile,
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
Poinfile will be generated based on the demo file name.
If the demo file is 'bkz_goldbhop.dem', the output file will be 'bkz_goldbhop.pts'

Usage:
  ./binary <path to demo>"
        )
    };

    let wrap = |demo_file_name: &str| {
        let demo_file_name = Path::new(demo_file_name);
        let demo = open_demo!(demo_file_name);
        let point_file_name = demo_file_name.file_stem().unwrap().to_str().unwrap();
        demo_to_pointfile(&demo, &point_file_name);
        println!("Pointfile successfully generated: {}", point_file_name);
    };

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => help(),
        2 => wrap(&args[1]),
        _ => help(),
    }
}
