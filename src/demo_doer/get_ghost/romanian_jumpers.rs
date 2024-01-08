use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{GhostFrame, GhostInfo};

// Order of appearance for serde.
#[derive(Serialize, Deserialize, Debug)]
struct RomanianJumpersGhostInfo {
    frames: Vec<RomanianJumpersGhostFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RomanianJumpersGhostFrame {
    #[serde(rename = "position")]
    origin: [f32; 3],
    #[serde(rename = "orientation")]
    viewangles: [f32; 2], // pitch yaw
    #[serde(rename = "length")]
    frametime: f32,
    time: f32, // total time
    buttons: u32,
}

pub fn romanian_jumpers_ghost_parse(filename: String, offset: f32) -> GhostInfo {
    let pathbuf = PathBuf::from(filename.to_owned());
    let file = match std::fs::read_to_string(&pathbuf) {
        Ok(file) => file,
        Err(_) => panic!("Cannot read file {}", filename),
    };

    let romanian_jumpers_ghost: RomanianJumpersGhostInfo = serde_json::from_str(&file).unwrap();

    // Convert romanian_jumpers_ghost to our normal ghost.
    GhostInfo {
        ghost_name: filename,
        entity_index: 0,
        use_frametime: true,
        frames: romanian_jumpers_ghost
            .frames
            .iter()
            .map(|ghost| GhostFrame {
                frametime: ghost.frametime,
                origin: [ghost.origin[0], -ghost.origin[2], ghost.origin[1]],
                viewangles: [ghost.viewangles[0], ghost.viewangles[1], 0.],
                sequence: None,
                frame: None,
                animtime: None,
                buttons: ghost.buttons.into(),
            })
            .collect(),
        ghost_anim_frame: 0.,
    }
}
