use nom::{
    bits,
    bits::complete::take as take_bit,
    bytes,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::char,
    combinator::{map, peek},
    multi::count,
    number::complete::{le_i32, le_u8},
    sequence::{terminated, tuple},
    AsChar, IResult,
};

use crate::types::*;
use crate::writer::*;

pub mod disconnect;
pub mod event;
// pub mod version;
// pub mod setview;
pub mod server_info;
pub mod user_message;
pub mod utils;

use utils::null_string;

pub trait NetMsgDoer<'a, T> {
    fn parse(i: &'a [u8]) -> IResult<&[u8], T>;
    fn write(i: T) -> Vec<u8>;
}

// Parser should not parse type.
// Writer however should write type.

// pub fn parse_netmsg_message(msg: &[u8]) -> Vec<NetMsgMessageBlock> {

// }

// fn what() {
//     parse_server_info()
// }
