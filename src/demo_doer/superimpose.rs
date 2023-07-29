use hldemo::{Demo, FrameData};

/// https://github.com/matthewearl/demsuperimpose/blob/master/demsuperimpose.py
use super::*;

struct GhostInfo<'a> {
    models: Vec<String>,
    // There's no manipulation with the data
    entity_baseline: &'a [u8],
    entity_updates: Vec<&'a [u8]>,
}

fn superimpose(demo: Demo) {
    for entry in demo.directory.entries {
        for frame in entry.frames {
            if let FrameData::NetMsg((_, data)) = frame.data {}
        }
    }
}
