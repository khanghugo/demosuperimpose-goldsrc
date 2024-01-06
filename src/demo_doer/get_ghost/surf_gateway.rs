use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{GhostFrame, GhostInfo};

// Order of appearance for serde.
#[derive(Serialize, Deserialize, Debug)]
struct SurfGatewayGhostInfo {
    name: String,
    authid: String,
    time: f32,
    startvel: [f32; 3],
    frames: Vec<SurfGatewayGhostFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SurfGatewayGhostFrame {
    origin: [f32; 3],
    viewangles: [f32; 3],
    moves: [f32; 3],
    buttons: u32,
    impulses: u32,
    frametime: u32, // This one is something else.
}

pub fn surf_gateway_ghost_parse(filename: String, offset: f32) -> GhostInfo {
    let pathbuf = PathBuf::from(filename.to_owned());
    let file = match std::fs::read_to_string(&pathbuf) {
        Ok(file) => file,
        Err(_) => panic!("Cannot read file {}", filename),
    };

    let surf_gateway_ghost: SurfGatewayGhostInfo = serde_json::from_str(&file).unwrap();

    // Convert surf_gateway_ghost to our normal ghost.
    GhostInfo {
        ghost_name: filename,
        entity_index: 0,
        use_frametime: false,
        frames: surf_gateway_ghost
            .frames
            .iter()
            .map(|ghost| GhostFrame {
                frametime: 0.,
                origin: ghost.origin,
                viewangles: ghost.viewangles,
                sequence: None,
                frame: None,
                animtime: None,
            })
            .collect(),
        ghost_anim_frame: 0.,
    }
}
