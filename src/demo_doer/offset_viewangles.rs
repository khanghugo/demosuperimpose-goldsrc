use hldemo::parse::frame;

use super::*;

/// Offset the yaw for `amount` starting at `over_end` but with `over_start` to smoothly change.
pub fn offset_yaw(demo: &mut Demo, over_start: usize, over_end: usize, amount: f32) {
    for (entry_idx, entry) in demo.directory.entries.iter_mut().enumerate() {
        if entry_idx == 0 {
            continue;
        }

        // Frame at over_end
        let (_, end_frame_viewangles) = search_client_data_frame(&entry.frames, over_end);
        let mut start_frame_viewangles: Option<[f32; 3]> = None;
        let range = over_end - over_start;

        for (frame_idx, frame) in entry.frames.iter_mut().enumerate() {
            match &mut frame.data {
                FrameData::NetMsg((_, data)) => {
                    if frame_idx < over_start {
                        continue;
                    }

                    if frame_idx >= over_start && start_frame_viewangles.is_none() {
                        start_frame_viewangles = Some(data.info.ref_params.viewangles);
                    }

                    if frame_idx > over_end {
                        // Overdue, just set it
                        data.info.ref_params.viewangles[1] += amount;
                    } else {
                        // Gradient change
                        let t = (frame_idx - over_start) as f32 / range as f32;
                        data.info.ref_params.viewangles[1] = (1. - t)
                            * start_frame_viewangles.unwrap()[1]
                            + t * ((end_frame_viewangles)[1] + amount);
                    }
                }
                _ => (),
            }
        }
    }
}

// Specify how far we search for the frame.
const SEARCH_RANGE: usize = 3;

fn search_client_data_frame(frames: &Vec<hldemo::Frame>, target: usize) -> (usize, [f32; 3]) {
    if frames.get(target - SEARCH_RANGE).is_none() || frames.get(target + SEARCH_RANGE).is_none() {
        panic!("Offset viewangles: Input frame for change over does not exist.")
    }

    for (frame_idx, frame) in frames[(target - SEARCH_RANGE)..(target + SEARCH_RANGE)]
        .iter()
        .enumerate()
    {
        match &frame.data {
            FrameData::NetMsg((_, data)) => {
                return (
                    target + frame_idx - SEARCH_RANGE,
                    data.info.ref_params.viewangles,
                );
            }
            _ => (),
        }
    }

    panic!("Offset viewangles: There are no client data frames in search range.")
}
