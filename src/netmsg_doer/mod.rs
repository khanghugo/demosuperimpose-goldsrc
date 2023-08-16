use nom::{
    bits,
    bits::complete::take as take_bit,
    bytes,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::char,
    combinator::{map, peek},
    multi::count,
    number::complete::{le_f32, le_i32, le_u16, le_u8},
    sequence::{terminated, tuple},
    AsChar, IResult,
};

use crate::types::*;
use crate::writer::*;

pub mod client_data;
pub mod delta_description;
pub mod disconnect;
pub mod event;
// pub mod version;
// pub mod setview;
pub mod print;
pub mod send_extra_info;
pub mod server_info;
pub mod time;
pub mod user_message;
pub mod utils;

use utils::{null_string, parse_delta, BitReader};

pub trait NetMsgDoer<'a, T> {
    /// Does not parse the type byte but only the message after that.
    fn parse(i: &'a [u8], delta_decoders: &mut DeltaDecoderTable) -> IResult<&'a [u8], T>;
    /// Must also write message type.
    fn write(i: T) -> Vec<u8>;
}

// Parser should not parse type.
// Writer however should write type.

// pub fn parse_netmsg_message(msg: &[u8]) -> Vec<NetMsgMessageBlock> {

// }

// fn what() {
//     parse_server_info()
// }
