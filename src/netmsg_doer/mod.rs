use std::{str::from_utf8, collections::HashMap};

use nom::{
    bits,
    bits::complete::take as take_bit,
    bytes,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::char,
    combinator::{map, peek, all_consuming, rest},
    multi::{count, many0},
    number::complete::{le_f32, le_i16, le_i32, le_i8, le_u16, le_u32, le_u8},
    sequence::{terminated, tuple},
    AsChar, IResult, Parser,
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

use self::{user_message::UserMessage, new_movevars::NewMovevars, print::Print, cd_track::CdTrack, send_extra_info::SendExtraInfo, new_user_msg::NewUserMsg, client_data::ClientData, delta_description::DeltaDescription, update_user_info::UpdateUserInfo, server_info::ServerInfo, stuff_text::StuffText, time::Time, set_view::SetView};

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

macro_rules! wrap_parse {
    ($input:ident, $parser:ident, $svc:ident, $dd:ident) => {
        {
            let ($input, res) = $parser::parse($input, $dd)?;
            ($input, Message::EngineMessage(EngineMessage::$svc(res)))
        }
    };
}

fn parse_single_netmsg<'a>(i: &'a [u8], delta_decoders: &mut HashMap<String, DeltaDecoder<'a>>) -> IResult<&'a [u8], Message<'a>> {
    let (i, type_) = le_u8(i)?;
    Ok(match MessageType::from(type_) {
        // 8 => wrap_parse!(i, Print, SvcPrint, delta_decoders),
        // 44 => wrap_parse!(i, )
        MessageType::UserMessage => {
            let (i, res) = UserMessage::parse(i, delta_decoders)?;
            (i, Message::UserMessage(res))
        },
        MessageType::EngineMessageType(engine_message_type) => {
            match engine_message_type {
                EngineMessageType::SvcBad => (i, Message::EngineMessage(EngineMessage::SvcBad)),
                // EngineMessageType::SvcNop => wrap_parse!(i, Nop, SvcNop, delta_decoders),
                // EngineMessageType::SvcDisconnect => wrap_parse!(i, Disconnect, SvcDisconnect, delta_decoders),
                // EngineMessageType::SvcEvent => wrap_parse!(i, Event, SvcEvent, delta_decoders),
                // EngineMessageType::SvcVersion => wrap_parse!(i, Version, SvcVersion, delta_decoders),
                EngineMessageType::SvcSetView => wrap_parse!(i, SetView, SvcSetView, delta_decoders),
                // EngineMessageType::SvcSound => wrap_parse!(i, Sound, SvcSound, delta_decoders),
                EngineMessageType::SvcTime => wrap_parse!(i, Time, SvcTime, delta_decoders),
                EngineMessageType::SvcPrint => wrap_parse!(i, Print, SvcPrint, delta_decoders),
                EngineMessageType::SvcStuffText => wrap_parse!(i, StuffText, SvcStuffText, delta_decoders),
                // EngineMessageType::SvcSetAngle => wrap_parse!(i, SetAngle, SvcSetAngle, delta_decoders),
                EngineMessageType::SvcServerInfo => wrap_parse!(i, ServerInfo, SvcServerInfo, delta_decoders),
                // EngineMessageType::SvcLightStyle => wrap_parse!(i, LightStyle, SvcLightStyle, delta_decoders),
                EngineMessageType::SvcUpdateUserInfo => wrap_parse!(i, UpdateUserInfo, SvcUpdateuserInfo, delta_decoders),
                EngineMessageType::SvcDeltaDescription => {
                    // Mutate delta_decoders here
                    let res = wrap_parse!(i, DeltaDescription, SvcDeltaDescription, delta_decoders);
                    if let Message::EngineMessage(EngineMessage::SvcDeltaDescription(SvcDeltaDescription { name, total_fields: _, fields })) = &res.1 {
                        delta_decoders.insert(from_utf8(name).unwrap().to_owned(), fields.to_vec());
                    };
                    res
                },
                EngineMessageType::SvcClientData => wrap_parse!(i, ClientData, SvcClientData, delta_decoders),
                // EngineMessageType::SvcStopsound => wrap_parse!(i, Stopsound, SvcStopsound, delta_decoders),
                // EngineMessageType::SvcPings => wrap_parse!(i, Pings, SvcPings, delta_decoders),
                // EngineMessageType::SvcParticle => wrap_parse!(i, Particle, SvcParticle, delta_decoders),
                // EngineMessageType::SvcDamage => wrap_parse!(i, Damage, SvcDamage, delta_decoders),
                // EngineMessageType::SvcSpawnStatic => wrap_parse!(i, SpawnStatic, SvcSpawnStatic, delta_decoders),
                // EngineMessageType::SvcEvenReliable => wrap_parse!(i, EvenReliable, SvcEvenReliable, delta_decoders),
                // EngineMessageType::SvcSpawnBaseline => wrap_parse!(i, SpawnBaseline, SvcSpawnBaseline, delta_decoders),
                // EngineMessageType::SvcTempEntity => wrap_parse!(i, TempEntity, SvcTempEntity, delta_decoders),
                // EngineMessageType::SvcSetPause => wrap_parse!(i, SetPause, SvcSetPause, delta_decoders),
                // EngineMessageType::SvcSignonNum => wrap_parse!(i, SignonNum, SvcSignonNum, delta_decoders),
                // EngineMessageType::SvcCenterPrint => wrap_parse!(i, CenterPrint, SvcCenterPrint, delta_decoders),
                // EngineMessageType::SvcKilledMonster => wrap_parse!(i, KilledMonster, SvcKilledMonster, delta_decoders),
                // EngineMessageType::SvcFoundSecret => wrap_parse!(i, FoundSecret, SvcFoundSecret, delta_decoders),
                // EngineMessageType::SvcSpawnStaticSound => wrap_parse!(i, SpawnStaticSound, SvcSpawnStaticSound, delta_decoders),
                // EngineMessageType::SvcIntermission => wrap_parse!(i, Intermission, SvcIntermission, delta_decoders),
                // EngineMessageType::SvcFinale => wrap_parse!(i, Finalie, SvcFinale, delta_decoders),
                EngineMessageType::SvcCdTrack => wrap_parse!(i, CdTrack, SvcCdTrack, delta_decoders),
                // EngineMessageType::SvcRestore => wrap_parse!(i, Restore, SvcRestore, delta_decoders),
                // EngineMessageType::SvcCutscene => wrap_parse!(i, Cutscene, SvcCutscene, delta_decoders),
                // EngineMessageType::SvcWeaponAnim => wrap_parse!(i, WeaponAnim, SvcWeaponAnim, delta_decoders),
                // EngineMessageType::SvcDecalName => wrap_parse!(i, DecalName, SvcDecalName, delta_decoders),
                // EngineMessageType::SvcRoomType => wrap_parse!(i, RoomType, SvcRoomType, delta_decoders),
                // EngineMessageType::SvcAddAngle => wrap_parse!(i, AddAngle, SvcAddAngle, delta_decoders),
                EngineMessageType::SvcNewUserMsg => wrap_parse!(i, NewUserMsg, SvcNewUserMsg, delta_decoders),
                // EngineMessageType::SvcPacketEntities => wrap_parse!(i, PacketEntities, SvcPacketEntities, delta_decoders),
                // EngineMessageType::SvcDeltaPacketEntities => wrap_parse!(i, DeltaPacketEntities, SvcDeltaPacketEntities, delta_decoders),
                // EngineMessageType::SvcChoke => wrap_parse!(i, Choke, SvcChoke, delta_decoders),
                // EngineMessageType::SvcResourceList => wrap_parse!(i, ResourceList, SvcResourceList, delta_decoders),
                EngineMessageType::SvcNewMoveVars => wrap_parse!(i, NewMovevars, SvcNewMoveVars, delta_decoders),
                // EngineMessageType::SvcResourceRequest => wrap_parse!(i, ResourceRequest, SvcResourceRequest, delta_decoders),
                // EngineMessageType::SvcCustomization => wrap_parse!(i, Customization, SvcCustomization, delta_decoders),
                // EngineMessageType::SvcCrosshairAngle => wrap_parse!(i, CrosshairAngle, SvcCrosshairAngle, delta_decoders),
                // EngineMessageType::SvcSoundFade => wrap_parse!(i, SoundFade, SvcSoundFade, delta_decoders),
                // EngineMessageType::SvcFileTxferFailed => wrap_parse!(i, FileTxferFailed, SvcFileTxferFailed, delta_decoders),
                // EngineMessageType::SvcHltv => wrap_parse!(i, Hltv, SvcHltv, delta_decoders),
                // EngineMessageType::SvcDirector => wrap_parse!(i, Director, SvcDirector, delta_decoders),
                // EngineMessageType::SvcVoiceInit => wrap_parse!(i, VoiceInit, SvcVoiceInit, delta_decoders),
                // EngineMessageType::SvcVoiceData => wrap_parse!(i, VoiceData, SvcVoiceData, delta_decoders),
                EngineMessageType::SvcSendExtraInfo => wrap_parse!(i, SendExtraInfo, SvcSendExtraInfo, delta_decoders),
                // EngineMessageType::SvcTimeScale => wrap_parse!(i, TimeScale, SvcTimeScale, delta_decoders),
                // EngineMessageType::SvcResourceLocation => wrap_parse!(i, ResourceLocation, SvcResourceLocation, delta_decoders),
                // EngineMessageType::SvcSendCvarValue => wrap_parse!(i, SendCvarValue, SvcSendCvarValue, delta_decoders),
                // EngineMessageType::SvcSendCvarValue2 => wrap_parse!(i, SendCvarValue2, SvcSendCvarValue2, delta_decoders),
                _ => (i, Message::EngineMessage(EngineMessage::SvcNop))
            }
        }
    })
}

pub fn parse_netmsg<'a>(i: &'a [u8], delta_decoders: &mut HashMap<String, DeltaDecoder<'a>>) -> IResult<&'a [u8], Vec<Message<'a>>> {
    let parser = move |i| parse_single_netmsg(i, delta_decoders);
    all_consuming(many0(parser))(i)
}

// How 2 read these things
// pub fn u8_slice_to_string(i: &[u8]) -> &str {
//     from_utf8(i).unwrap()
// }