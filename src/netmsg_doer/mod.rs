use nom::{
    bits,
    bits::complete::take as take_bit,
    bytes,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::char,
    combinator::{map, peek},
    multi::count,
    number::complete::{le_f32, le_i16, le_i32, le_i8, le_u16, le_u32, le_u8},
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
pub mod cd_track;
pub mod new_movevars;
pub mod new_user_msg;
pub mod print;
pub mod send_extra_info;
pub mod server_info;
pub mod set_view;
pub mod stuff_text;
pub mod time;
pub mod update_user_info;
pub mod user_message;
pub mod utils;

use utils::{null_string, parse_delta, BitReader};

/*
use super::*;

pub struct What {}
impl<'a> NetMsgDoer<'a, Svc<'a>> for What {
    fn parse(i: &'a [u8], delta_decoders: &mut DeltaDecoderTable) -> IResult<&'a [u8], Svc<'a>> {
        todo!()
    }

    fn write(i: Svc) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(0u8);

        writer.data
    }
}
*/
pub trait NetMsgDoer<'a, T> {
    /// Does not parse the type byte but only the message after that.
    fn parse(i: &'a [u8], delta_decoders: &mut DeltaDecoderTable) -> IResult<&'a [u8], T>;
    /// Must also write message type.
    fn write(i: T) -> Vec<u8>;
}
