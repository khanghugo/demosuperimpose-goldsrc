use std::{collections::HashMap, str::from_utf8};

use nom::{
    bits,
    bits::complete::take as take_bit,
    bytes,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::char,
    combinator::{all_consuming, cond, flat_map, map, peek, rest},
    multi::{count, many0},
    number::complete::{le_f32, le_i16, le_i32, le_i8, le_u16, le_u32, le_u8},
    sequence::{terminated, tuple},
    AsChar, IResult, Parser,
};

use crate::types::*;
use crate::writer::*;

pub mod add_angle;
pub mod cd_track;
pub mod center_print;
pub mod client_data;
pub mod crosshair_angle;
pub mod customization;
pub mod cutscene;
pub mod decal_name;
pub mod delta_description;
pub mod delta_packet_entities;
pub mod director;
pub mod disconnect;
pub mod event;
pub mod event_reliable;
pub mod file_txfer_failed;
pub mod finale;
pub mod hltv;
pub mod light_style;
pub mod new_movevars;
pub mod new_user_msg;
pub mod packet_entities;
pub mod particle;
pub mod pings;
pub mod print;
pub mod resource_list;
pub mod resource_location;
pub mod resource_request;
pub mod restore;
pub mod room_type;
pub mod send_cvar_value;
pub mod send_cvar_value_2;
pub mod send_extra_info;
pub mod server_info;
pub mod set_angle;
pub mod set_pause;
pub mod set_view;
pub mod sign_on_num;
pub mod sound;
pub mod sound_fade;
pub mod spawn_baseline;
pub mod spawn_static;
pub mod spawn_static_sound;
pub mod stop_sound;
pub mod stuff_text;
pub mod temp_entity;
pub mod time;
pub mod time_scale;
pub mod update_user_info;
pub mod user_message;
pub mod utils;
pub mod version;
pub mod voice_data;
pub mod voice_init;
pub mod weapon_anim;

use utils::{null_string, parse_delta, BitReader};

use self::{
    add_angle::AddAngle, cd_track::CdTrack, center_print::CenterPrint, client_data::ClientData,
    crosshair_angle::CrosshairAngle, customization::Customization, cutscene::Cutscene,
    decal_name::DecalName, delta_description::DeltaDescription,
    delta_packet_entities::DeltaPacketEntities, director::Director, disconnect::Disconnect,
    event::Event, event_reliable::EventReliable, file_txfer_failed::FileTxferFailed,
    finale::Finale, hltv::Hltv, light_style::LightStyle, new_movevars::NewMovevars,
    new_user_msg::NewUserMsg, packet_entities::PacketEntities, particle::Particle, pings::Pings,
    print::Print, resource_list::ResourceList, resource_location::ResourceLocation,
    resource_request::ResourceRequest, restore::Restore, room_type::RoomType,
    send_cvar_value::SendCvarValue, send_cvar_value_2::SendCvarValue2,
    send_extra_info::SendExtraInfo, server_info::ServerInfo, set_angle::SetAngle,
    set_pause::SetPause, set_view::SetView, sign_on_num::SignOnNum, sound::Sound,
    sound_fade::SoundFade, spawn_baseline::SpawnBaseline, spawn_static::SpawnStatic,
    spawn_static_sound::SpawnStaticSound, stop_sound::StopSound, stuff_text::StuffText,
    temp_entity::TempEntity, time::Time, time_scale::TimeScale, update_user_info::UpdateUserInfo,
    user_message::UserMessage, version::Version, voice_data::VoiceData, voice_init::VoiceInit,
    weapon_anim::WeaponAnim,
};

/*
use super::*;

pub struct What {}
impl<'a> NetMsgDoer<'a, Svc> for What {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], Svc> {
        todo!()
    }

    fn write(i: Svc) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::Svc as u8);

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

// Should have done one differently for normal netmessage and ones with delta as well....
pub trait UserMessageDoer<'a, T> {
    /// Does not parse the type byte but only the message after that.
    fn parse(
        i: &'a [u8],
        id: u8,
        custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
    ) -> IResult<&'a [u8], T>;
    /// Must also write message type.
    fn write(i: T, custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>) -> Vec<u8>;
}

macro_rules! wrap_parse {
    ($input:ident, $parser:ident, $svc:ident, $dd:ident) => {{
        let ($input, res) = $parser::parse($input, $dd)?;
        ($input, Message::EngineMessage(EngineMessage::$svc(res)))
    }};
}

fn parse_single_netmsg<'a>(
    i: &'a [u8],
    delta_decoders: &mut HashMap<String, DeltaDecoder>,
    custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
) -> IResult<&'a [u8], Message<'a>> {
    // println!("{:?}", i);

    let (i, type_) = le_u8(i)?;
    let (i, res) = match MessageType::from(type_) {
        MessageType::UserMessage => {
            let (i, res) = UserMessage::parse(i, type_, custom_messages)?;
            (i, Message::UserMessage(res))
        }
        MessageType::EngineMessageType(engine_message_type) => {
            match engine_message_type {
                EngineMessageType::SvcBad => (i, Message::EngineMessage(EngineMessage::SvcBad)),
                EngineMessageType::SvcNop => (i, Message::EngineMessage(EngineMessage::SvcNop)),
                EngineMessageType::SvcDisconnect => {
                    wrap_parse!(i, Disconnect, SvcDisconnect, delta_decoders)
                }
                EngineMessageType::SvcEvent => wrap_parse!(i, Event, SvcEvent, delta_decoders),
                EngineMessageType::SvcVersion => {
                    wrap_parse!(i, Version, SvcVersion, delta_decoders)
                }
                EngineMessageType::SvcSetView => {
                    wrap_parse!(i, SetView, SvcSetView, delta_decoders)
                }
                EngineMessageType::SvcSound => wrap_parse!(i, Sound, SvcSound, delta_decoders),
                EngineMessageType::SvcTime => wrap_parse!(i, Time, SvcTime, delta_decoders),
                EngineMessageType::SvcPrint => wrap_parse!(i, Print, SvcPrint, delta_decoders),
                EngineMessageType::SvcStuffText => {
                    wrap_parse!(i, StuffText, SvcStuffText, delta_decoders)
                }
                EngineMessageType::SvcSetAngle => {
                    wrap_parse!(i, SetAngle, SvcSetAngle, delta_decoders)
                }
                EngineMessageType::SvcServerInfo => {
                    wrap_parse!(i, ServerInfo, SvcServerInfo, delta_decoders)
                }
                EngineMessageType::SvcLightStyle => {
                    wrap_parse!(i, LightStyle, SvcLightStyle, delta_decoders)
                }
                EngineMessageType::SvcUpdateUserInfo => {
                    wrap_parse!(i, UpdateUserInfo, SvcUpdateUserInfo, delta_decoders)
                }
                EngineMessageType::SvcDeltaDescription => {
                    // Mutate delta_decoders here
                    let res = wrap_parse!(i, DeltaDescription, SvcDeltaDescription, delta_decoders);
                    if let Message::EngineMessage(EngineMessage::SvcDeltaDescription(
                        SvcDeltaDescription {
                            name,
                            total_fields: _,
                            fields,
                            clone: _,
                        },
                    )) = &res.1
                    {
                        delta_decoders.insert(from_utf8(name).unwrap().to_owned(), fields.to_vec());
                    };
                    res
                }
                EngineMessageType::SvcClientData => {
                    wrap_parse!(i, ClientData, SvcClientData, delta_decoders)
                }
                EngineMessageType::SvcStopSound => {
                    wrap_parse!(i, StopSound, SvcStopSound, delta_decoders)
                }
                EngineMessageType::SvcPings => wrap_parse!(i, Pings, SvcPings, delta_decoders),
                EngineMessageType::SvcParticle => {
                    wrap_parse!(i, Particle, SvcParticle, delta_decoders)
                }
                EngineMessageType::SvcDamage => {
                    (i, Message::EngineMessage(EngineMessage::SvcDamage))
                }
                EngineMessageType::SvcSpawnStatic => {
                    wrap_parse!(i, SpawnStatic, SvcSpawnStatic, delta_decoders)
                }
                EngineMessageType::SvcEventReliable => {
                    wrap_parse!(i, EventReliable, SvcEventReliable, delta_decoders)
                }
                EngineMessageType::SvcSpawnBaseline => {
                    wrap_parse!(i, SpawnBaseline, SvcSpawnBaseline, delta_decoders)
                }
                EngineMessageType::SvcTempEntity => {
                    wrap_parse!(i, TempEntity, SvcTempEntity, delta_decoders)
                }
                EngineMessageType::SvcSetPause => {
                    wrap_parse!(i, SetPause, SvcSetPause, delta_decoders)
                }
                EngineMessageType::SvcSignOnNum => {
                    wrap_parse!(i, SignOnNum, SvcSignOnNum, delta_decoders)
                }
                EngineMessageType::SvcCenterPrint => {
                    wrap_parse!(i, CenterPrint, SvcCenterPrint, delta_decoders)
                }
                EngineMessageType::SvcKilledMonster => {
                    (i, Message::EngineMessage(EngineMessage::SvcKilledMonster))
                }
                EngineMessageType::SvcFoundSecret => {
                    (i, Message::EngineMessage(EngineMessage::SvcFoundSecret))
                }
                EngineMessageType::SvcSpawnStaticSound => {
                    wrap_parse!(i, SpawnStaticSound, SvcSpawnStaticSound, delta_decoders)
                }
                EngineMessageType::SvcIntermission => {
                    (i, Message::EngineMessage(EngineMessage::SvcIntermission))
                }
                EngineMessageType::SvcFinale => wrap_parse!(i, Finale, SvcFinale, delta_decoders),
                EngineMessageType::SvcCdTrack => {
                    wrap_parse!(i, CdTrack, SvcCdTrack, delta_decoders)
                }
                EngineMessageType::SvcRestore => {
                    wrap_parse!(i, Restore, SvcRestore, delta_decoders)
                }
                EngineMessageType::SvcCutscene => {
                    wrap_parse!(i, Cutscene, SvcCutscene, delta_decoders)
                }
                EngineMessageType::SvcWeaponAnim => {
                    wrap_parse!(i, WeaponAnim, SvcWeaponAnim, delta_decoders)
                }
                EngineMessageType::SvcDecalName => {
                    wrap_parse!(i, DecalName, SvcDecalName, delta_decoders)
                }
                EngineMessageType::SvcRoomType => {
                    wrap_parse!(i, RoomType, SvcRoomType, delta_decoders)
                }
                EngineMessageType::SvcAddAngle => {
                    wrap_parse!(i, AddAngle, SvcAddAngle, delta_decoders)
                }
                EngineMessageType::SvcNewUserMsg => {
                    let res = wrap_parse!(i, NewUserMsg, SvcNewUserMsg, delta_decoders);

                    if let Message::EngineMessage(EngineMessage::SvcNewUserMsg(ref msg)) = res.1 {
                        custom_messages.remove(&msg.index);
                        custom_messages.insert(msg.index, msg.clone());
                    }

                    res
                }
                EngineMessageType::SvcPacketEntities => {
                    wrap_parse!(i, PacketEntities, SvcPacketEntities, delta_decoders)
                }
                EngineMessageType::SvcDeltaPacketEntities => wrap_parse!(
                    i,
                    DeltaPacketEntities,
                    SvcDeltaPacketEntities,
                    delta_decoders
                ),
                EngineMessageType::SvcChoke => (i, Message::EngineMessage(EngineMessage::SvcChoke)),
                EngineMessageType::SvcResourceList => {
                    wrap_parse!(i, ResourceList, SvcResourceList, delta_decoders)
                }
                EngineMessageType::SvcNewMoveVars => {
                    wrap_parse!(i, NewMovevars, SvcNewMovevars, delta_decoders)
                }
                EngineMessageType::SvcResourceRequest => {
                    wrap_parse!(i, ResourceRequest, SvcResourceRequest, delta_decoders)
                }
                EngineMessageType::SvcCustomization => {
                    wrap_parse!(i, Customization, SvcCustomization, delta_decoders)
                }
                EngineMessageType::SvcCrosshairAngle => {
                    wrap_parse!(i, CrosshairAngle, SvcCrosshairAngle, delta_decoders)
                }
                EngineMessageType::SvcSoundFade => {
                    wrap_parse!(i, SoundFade, SvcSoundFade, delta_decoders)
                }
                EngineMessageType::SvcFileTxferFailed => {
                    wrap_parse!(i, FileTxferFailed, SvcFileTxferFailed, delta_decoders)
                }
                EngineMessageType::SvcHltv => wrap_parse!(i, Hltv, SvcHltv, delta_decoders),
                EngineMessageType::SvcDirector => {
                    wrap_parse!(i, Director, SvcDirector, delta_decoders)
                }
                EngineMessageType::SvcVoiceInit => {
                    wrap_parse!(i, VoiceInit, SvcVoiceInit, delta_decoders)
                }
                EngineMessageType::SvcVoiceData => {
                    wrap_parse!(i, VoiceData, SvcVoiceData, delta_decoders)
                }
                EngineMessageType::SvcSendExtraInfo => {
                    wrap_parse!(i, SendExtraInfo, SvcSendExtraInfo, delta_decoders)
                }
                EngineMessageType::SvcTimeScale => {
                    wrap_parse!(i, TimeScale, SvcTimeScale, delta_decoders)
                }
                EngineMessageType::SvcResourceLocation => {
                    wrap_parse!(i, ResourceLocation, SvcResourceLocation, delta_decoders)
                }
                EngineMessageType::SvcSendCvarValue => {
                    wrap_parse!(i, SendCvarValue, SvcSendCvarValue, delta_decoders)
                }
                EngineMessageType::SvcSendCvarValue2 => {
                    wrap_parse!(i, SendCvarValue2, SvcSendCvarValue2, delta_decoders)
                }
                _ => (i, Message::EngineMessage(EngineMessage::SvcNop)),
            }
        }
    };

    // println!("{:?}", res);

    Ok((i, res))
}

pub fn parse_netmsg<'a>(
    i: &'a [u8],
    delta_decoders: &mut HashMap<String, DeltaDecoder>,
    custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
) -> IResult<&'a [u8], Vec<Message<'a>>> {
    let parser = move |i| parse_single_netmsg(i, delta_decoders, custom_messages);
    all_consuming(many0(parser))(i)
}

pub fn write_single_netmsg<'a>(
    i: Message<'a>,
    _: &mut HashMap<String, DeltaDecoder>,
    custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
) -> Vec<u8> {
    match i {
        Message::UserMessage(what) => UserMessage::write(what, custom_messages),
        Message::EngineMessage(what) => match what {
            EngineMessage::SvcBad => vec![EngineMessageType::SvcBad as u8],
            EngineMessage::SvcNop => vec![EngineMessageType::SvcNop as u8],
            EngineMessage::SvcDisconnect(i) => Disconnect::write(i),
            EngineMessage::SvcEvent(i) => Event::write(i),
            EngineMessage::SvcVersion(i) => Version::write(i),
            EngineMessage::SvcSetView(i) => SetView::write(i),
            EngineMessage::SvcSound(i) => Sound::write(i),
            EngineMessage::SvcTime(i) => Time::write(i),
            EngineMessage::SvcPrint(i) => Print::write(i),
            EngineMessage::SvcStuffText(i) => StuffText::write(i),
            EngineMessage::SvcSetAngle(i) => SetAngle::write(i),
            EngineMessage::SvcServerInfo(i) => ServerInfo::write(i),
            EngineMessage::SvcLightStyle(i) => LightStyle::write(i),
            EngineMessage::SvcUpdateUserInfo(i) => UpdateUserInfo::write(i),
            EngineMessage::SvcDeltaDescription(i) => DeltaDescription::write(i),
            EngineMessage::SvcClientData(i) => ClientData::write(i),
            EngineMessage::SvcStopSound(i) => StopSound::write(i),
            EngineMessage::SvcPings(i) => Pings::write(i),
            EngineMessage::SvcParticle(i) => Particle::write(i),
            EngineMessage::SvcDamage => vec![EngineMessageType::SvcDamage as u8],
            EngineMessage::SvcSpawnStatic(i) => SpawnStatic::write(i),
            EngineMessage::SvcEventReliable(i) => EventReliable::write(i),
            EngineMessage::SvcSpawnBaseline(i) => SpawnBaseline::write(i),
            EngineMessage::SvcTempEntity(i) => TempEntity::write(i),
            EngineMessage::SvcSetPause(i) => SetPause::write(i),
            EngineMessage::SvcSignOnNum(i) => SignOnNum::write(i),
            EngineMessage::SvcCenterPrint(i) => CenterPrint::write(i),
            EngineMessage::SvcKilledMonster => vec![EngineMessageType::SvcKilledMonster as u8],
            EngineMessage::SvcFoundSecret => vec![EngineMessageType::SvcFoundSecret as u8],
            EngineMessage::SvcSpawnStaticSound(i) => SpawnStaticSound::write(i),
            EngineMessage::SvcIntermission => vec![EngineMessageType::SvcIntermission as u8],
            EngineMessage::SvcFinale(i) => Finale::write(i),
            EngineMessage::SvcCdTrack(i) => CdTrack::write(i),
            EngineMessage::SvcRestore(i) => Restore::write(i),
            EngineMessage::SvcCutscene(i) => Cutscene::write(i),
            EngineMessage::SvcWeaponAnim(i) => WeaponAnim::write(i),
            EngineMessage::SvcDecalName(i) => DecalName::write(i),
            EngineMessage::SvcRoomType(i) => RoomType::write(i),
            EngineMessage::SvcAddAngle(i) => AddAngle::write(i),
            EngineMessage::SvcNewUserMsg(i) => NewUserMsg::write(i),
            EngineMessage::SvcPacketEntities(i) => PacketEntities::write(i),
            EngineMessage::SvcDeltaPacketEntities(i) => DeltaPacketEntities::write(i),
            EngineMessage::SvcChoke => vec![EngineMessageType::SvcChoke as u8],
            EngineMessage::SvcResourceList(i) => ResourceList::write(i),
            EngineMessage::SvcNewMovevars(i) => NewMovevars::write(i),
            EngineMessage::SvcResourceRequest(i) => ResourceRequest::write(i),
            EngineMessage::SvcCustomization(i) => Customization::write(i),
            EngineMessage::SvcCrosshairAngle(i) => CrosshairAngle::write(i),
            EngineMessage::SvcSoundFade(i) => SoundFade::write(i),
            EngineMessage::SvcFileTxferFailed(i) => FileTxferFailed::write(i),
            EngineMessage::SvcHltv(i) => Hltv::write(i),
            EngineMessage::SvcDirector(i) => Director::write(i),
            EngineMessage::SvcVoiceInit(i) => VoiceInit::write(i),
            EngineMessage::SvcVoiceData(i) => VoiceData::write(i),
            EngineMessage::SvcSendExtraInfo(i) => SendExtraInfo::write(i),
            EngineMessage::SvcTimeScale(i) => TimeScale::write(i),
            EngineMessage::SvcResourceLocation(i) => ResourceLocation::write(i),
            EngineMessage::SvcSendCvarValue(i) => SendCvarValue::write(i),
            EngineMessage::SvcSendCvarValue2(i) => SendCvarValue2::write(i),
        },
    }
}

pub fn write_netmsg<'a>(
    i: Vec<Message<'a>>,
    delta_decoders: &mut HashMap<String, DeltaDecoder>,
    custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    for message in i {
        res.append(&mut write_single_netmsg(
            message,
            delta_decoders,
            custom_messages,
        ));
    }
    res
}
