use super::*;

/// Offset the yaw for `amount` starting at `over_end` but with `over_start` to smoothly change.
pub fn offset_yaw(demo: &mut Demo, over_start: usize, over_end: usize, amount: f32) {
    for (entry_idx, entry) in demo.directory.entries.iter_mut().enumerate() {
        if entry_idx == 0 {
            continue;
        }

        // Frame at over_end
        // Override over_end because we have more accurate number.
        let (over_end, end_frame_viewangles) = search_client_data_frame(&entry.frames, over_end);
        let mut start_frame_viewangles: Option<[f32; 3]> = None;
        let mut range = over_end - over_start;
        let mut over_start = over_start;

        for (frame_idx, frame) in entry.frames.iter_mut().enumerate() {
            match &mut frame.data {
                FrameData::NetMsg((_, data)) => {
                    if frame_idx < over_start {
                        continue;
                    }

                    if frame_idx >= over_start && start_frame_viewangles.is_none() {
                        start_frame_viewangles = Some(data.info.ref_params.viewangles);

                        over_start = frame_idx;
                        range = over_end - over_start;
                    }

                    if frame_idx >= over_end {
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

/// Mirror pitch around where the player looks at. 30 -> 150. -30 -> -150
pub fn mirror_pitch(demo: &mut Demo, over_start: usize, over_end: usize) {
    for (entry_idx, entry) in demo.directory.entries.iter_mut().enumerate() {
        if entry_idx == 0 {
            continue;
        }

        // Frame at over_end
        // Override over_end because we have more accurate number.
        let (over_end, end_frame_viewangles) = search_client_data_frame(&entry.frames, over_end);
        let mut start_frame_viewangles: Option<[f32; 3]> = None;
        let mut range = over_end - over_start;
        let mut over_start = over_start;

        for (frame_idx, frame) in entry.frames.iter_mut().enumerate() {
            match &mut frame.data {
                FrameData::NetMsg((_, data)) => {
                    if frame_idx < over_start {
                        continue;
                    }

                    if frame_idx >= over_start && start_frame_viewangles.is_none() {
                        start_frame_viewangles = Some(data.info.ref_params.viewangles);

                        over_start = frame_idx;
                        range = over_end - over_start;
                    }

                    if frame_idx >= over_end {
                        // Overdue, just set it
                        data.info.ref_params.viewangles[0] =
                            180. - data.info.ref_params.viewangles[0];
                    } else {
                        // Gradient change
                        let t = (frame_idx - over_start) as f32 / range as f32;
                        data.info.ref_params.viewangles[0] = (1. - t)
                            * start_frame_viewangles.unwrap()[0]
                            + t * (180. - end_frame_viewangles[0]);
                    }
                }
                _ => (),
            }
        }
    }
}

/// Scalar should best be a rounded number.
///
/// So if we have 1, it means it will rotate at most 1 revolution. 2 is 2 revs, and so on.
fn scalar_complete_rotation(
    demo: &mut Demo,
    start: usize,
    end: usize,
    scalar: f32,
    viewangles_index: usize,
) {
    for (entry_idx, entry) in demo.directory.entries.iter_mut().enumerate() {
        if entry_idx == 0 {
            continue;
        }

        // Frame at over_end
        let (end, end_frame_viewangles) = search_client_data_frame(&entry.frames, end);
        let mut start_frame_viewangles: Option<[f32; 3]> = None;
        let mut length: Option<f32> = None;
        let mut range = end - start;
        let mut start = start;

        for (frame_idx, frame) in entry.frames.iter_mut().enumerate() {
            match &mut frame.data {
                FrameData::NetMsg((_, data)) => {
                    if frame_idx < start {
                        continue;
                    }

                    if frame_idx >= start && start_frame_viewangles.is_none() {
                        let sign = if scalar.is_sign_positive() { 1. } else { -1. };

                        start_frame_viewangles = Some(data.info.ref_params.viewangles);
                        length = if scalar.is_sign_positive() {
                            Some(
                                (360. * scalar.abs().floor()
                                    - start_frame_viewangles.unwrap()[viewangles_index]
                                    + end_frame_viewangles[viewangles_index])
                                    * sign,
                            )
                        } else {
                            Some(
                                (360. * scalar.abs().floor()
                                    + start_frame_viewangles.unwrap()[viewangles_index]
                                    - end_frame_viewangles[viewangles_index])
                                    * sign,
                            )
                        };

                        // Reassign for correct interpolation
                        start = frame_idx;
                        range = end - start;
                    }

                    if frame_idx >= start && frame_idx < end && length.is_some() {
                        // Gradient change
                        let t = (frame_idx - start) as f32 / range as f32;
                        // `length` says how much we spin, so we cannot end with length but something plus length.
                        // Because we start with `start`, so it ends with `start` offset by `length`, which is `end_frame`.
                        data.info.ref_params.viewangles[viewangles_index] = (1. - t)
                            * start_frame_viewangles.unwrap()[viewangles_index]
                            + t * (start_frame_viewangles.unwrap()[viewangles_index]
                                + length.unwrap());
                    }
                }
                _ => (),
            }
        }
    }
}

/// Generic flip. Scalar is to go how fast. For frontflip and backflip.
pub fn scalar_flip(demo: &mut Demo, start: usize, end: usize, scalar: f32) {
    scalar_complete_rotation(demo, start, end, scalar, 0);
}

/// Rotate pitch by around 360 degrees forward from `start` pitch to `end` pitch.
pub fn front_flip(demo: &mut Demo, start: usize, end: usize) {
    scalar_flip(demo, start, end, 1.);
}

/// Frontflip but is backflip
pub fn back_flip(demo: &mut Demo, start: usize, end: usize) {
    scalar_flip(demo, start, end, -1.)
}

pub fn scalar_spin(demo: &mut Demo, start: usize, end: usize, scalar: f32) {
    scalar_complete_rotation(demo, start, end, scalar, 1);
}

pub fn spin_left(demo: &mut Demo, start: usize, end: usize) {
    scalar_spin(demo, start, end, 1.);
}

pub fn spin_right(demo: &mut Demo, start: usize, end: usize) {
    scalar_spin(demo, start, end, -1.)
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
