use demosuperimpose_goldsrc::netmsg_doer::{
    parse_netmsg,
    utils::{get_initial_delta, BitSliceCast},
    write_netmsg,
};
use demosuperimpose_goldsrc::types::*;

use std::collections::HashMap;

use hldemo::{Demo, FrameData};

pub mod add_debug;
pub mod example;
pub mod get_ghost;
pub mod ghost_to_demo;
pub mod kz_stats;
pub mod netmsg_rewrite_test;
pub mod offset_viewangles;
pub mod remove_entities;
pub mod superimpose;

// use bitvec::bitvec;
// use bitvec::prelude::*;

use crate::write_demo;

use std::fs::File;
use std::io::Read;
