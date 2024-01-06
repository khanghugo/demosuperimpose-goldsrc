use std::fs::File;
use std::io::Read;
use std::{io::Write, path::PathBuf};

use crate::demo_doer::get_ghost::romanian_jumpers::romanian_jumpers_ghost_parse;
use crate::{
    demo_doer::get_ghost::{
        demo::demo_ghost_parse, simen::simen_ghost_parse, surf_gateway::surf_gateway_ghost_parse,
    },
    open_demo,
};

use self::types::GhostFrame;
use self::types::GhostInfo;

mod demo;
mod romanian_jumpers;
mod simen;
mod surf_gateway;

mod types;

pub fn get_ghost(others: &Vec<(String, f32)>) -> Vec<GhostInfo> {
    others
        .iter()
        .enumerate()
        .map(|(index, (filename, offset))| {
            let pathbuf = PathBuf::from(filename);

            print!(
                "\rParsing {} ({}/{})    ",
                filename,
                index + 1,
                others.len()
            );
            std::io::stdout().flush().unwrap();

            let ghost = if pathbuf.to_str().unwrap().ends_with(".dem") {
                let demo = open_demo!(filename);
                demo_ghost_parse(filename, demo, *offset)
            } else if pathbuf.to_str().unwrap().ends_with(".simen.txt") {
                // Either this, or use enum in main file.
                simen_ghost_parse(filename.to_owned(), *offset)
            } else if pathbuf.to_str().unwrap().ends_with(".sg.json") {
                // Surf Gateway
                surf_gateway_ghost_parse(filename.to_owned(), *offset)
            } else if pathbuf.to_str().unwrap().ends_with(".rj.json") {
                // Romanian-Jumprs
                romanian_jumpers_ghost_parse(filename.to_owned(), *offset)
            } else {
                println!("");
                panic!("File \"{}\" does not use supported extension.", filename);
            };

            ghost
        })
        .collect()
}
