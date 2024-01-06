use super::*;

use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{multispace0, newline, space0, u32},
    combinator::{all_consuming, map, opt, recognize},
    multi::separated_list0,
    number::complete::float as _float,
    sequence::{delimited, preceded, tuple},
    IResult,
};

struct SimenGhostFrame {
    frame: GhostFrame,
    velocity: [f32; 3],
    button: u32,
    moves: [f32; 2],
}

pub fn simen_ghost_parse(filename: String, offset: f32) -> GhostInfo {
    let pathbuf = PathBuf::from(filename.to_owned());
    let file = match std::fs::read_to_string(&pathbuf) {
        Ok(file) => file,
        Err(_) => panic!("Cannot read file {}", filename),
    };

    let res = match map(
        preceded(
            simen_wrbot_header,
            all_consuming(delimited(
                opt(multispace0),
                separated_list0(
                    newline,
                    // Conversion from simen ghost to our generic ghost. Maybe we can add more things down the line.
                    map(simen_wrbot_line, |simen_ghost| simen_ghost.frame),
                ),
                opt(multispace0),
            )),
        ),
        |frames| GhostInfo {
            ghost_name: filename.to_owned(),
            entity_index: 0,
            frames,
            ghost_anim_frame: 0.,
        },
    )(&file)
    {
        Ok(res) => res.1,
        Err(_) => panic!("Cannot parse file {}", filename),
    };

    res
}

fn simen_wrbot_header(i: &str) -> IResult<&str, u8> {
    map(
        tuple((
            skip_line, // Time
            skip_line, // Name
            skip_line, // SteamID
            skip_line, // Date
            skip_line, // Location
            skip_line, // ??
        )),
        |_| 0u8,
    )(i)
}

fn simen_wrbot_line(i: &str) -> IResult<&str, SimenGhostFrame> {
    map(
        tuple((
            float, float, float, float, float, float, float, float, u32, float, float,
        )),
        |(pitch, yaw, posx, posy, posz, velx, vely, velz, button, move1, move2)| SimenGhostFrame {
            frame: GhostFrame {
                origin: [posx, posy, posz],
                viewangles: [pitch, yaw, 0.],
                sequence: None,
                frame: None,
                animtime: None,
            },
            velocity: [velx, vely, velz],
            button,
            moves: [move1, move2],
        },
    )(i)
}

fn skip_line(i: &str) -> IResult<&str, u8> {
    map(tuple((take_till(|c| c == '\n'), take(1usize))), |what| 0u8)(i)
}

fn signed_float(i: &str) -> IResult<&str, f32> {
    map(recognize(preceded(opt(tag("-")), _float)), |what: &str| {
        what.parse().unwrap()
    })(i)
}

pub fn float(i: &str) -> IResult<&str, f32> {
    preceded(space0, signed_float)(i)
}
