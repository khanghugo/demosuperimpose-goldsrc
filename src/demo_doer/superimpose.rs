use hldemo::{Demo, FrameData};

/// https://github.com/matthewearl/demsuperimpose/blob/master/demsuperimpose.py
use super::*;

// Parse individual demos and

struct GhostInfo<'a> {
    models: Vec<String>,
    entity_baseline: &'a [u8],
    entity_updates: Vec<&'a [u8]>,
}

pub struct ImposerDemo<'a> {
    pub demo: &'a Demo<'a>,
    pub offset: f32,
}

fn superimpose(main_demo: &mut Demo, imposers: Vec<ImposerDemo>) {
    for entry in &mut main_demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {}
        }
    }
}
