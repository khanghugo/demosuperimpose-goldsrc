// pub struct NetMsgMessageBlock<'a> {
//     pub type_: NetMsgMessageData,
//     pub data: NetMsgMessageData,
// }

use std::{collections::HashMap, ops::BitAnd};

use bitvec::{slice::BitSlice, vec::BitVec};

pub struct DeltaFieldDecoder<'a> {
    pub bits: u32,
    pub divisor: f32,
    pub flags: DeltaType,
    pub name: &'a [u8],
    pub offset: i32,
    pub pre_multiplier: i32,
    pub size: u32,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum DeltaType {
    Byte = 1,
    Short = 1 << 1,
    Float = 1 << 2,
    Integer = 1 << 3,
    Angle = 1 << 4,
    TimeWindow8 = 1 << 5,
    TimeWindowBig = 1 << 6,
    String = 1 << 7,
    Signed = 1 << 31,
}

// impl BitAnd for DeltaType {
//     type Output = u32;

//     fn bitand(self, rhs: Self) -> Self::Output {
//         self & rhs
//     }
// }

struct delta_s {
    dynamic: i32,
    field_count: i32,
    conditional_encode_name: [u8; 32],
    // TODO
}

pub struct delta_description_s {
    pub field_type: i32,
    pub field_name: [u8; 32],
    pub field_offset: i32,
    pub field_size: i16,
    pub significant_bits: i32,
    pub premultiply: f32,
    pub postmultiply: f32,
    pub flags: i16,
    pub stats: delta_stats_t,
}

pub struct delta_stats_t {
    pub send_count: i32,
    pub received_count: i32,
}

#[derive(Debug)]
/// A simplified struct of delta_description_s
///
/// Lots of info end up unused.
#[derive(Clone)]
pub struct DeltaDecoderS<'a> {
    pub name: &'a [u8],
    pub bits: u32,
    pub divisor: f32,
    pub flags: u32,
}

pub type Delta = HashMap<String, Vec<u8>>;
pub type DeltaDecoder<'a> = Vec<DeltaDecoderS<'a>>;
pub type DeltaDecoderTable<'a> = HashMap<String, DeltaDecoder<'a>>;

pub type BitType = BitVec<u8>;

// pub struct delta_description_t<'a> {
//     flags: u32,
//     name:
// }

/// UserMessage
#[derive(Debug)]
pub struct NetMsgUserMessage<'a> {
    pub message: &'a [u8],
}

/// SVC_BAD 0
// #[derive(Debug)]

/// SVC_NOP 1
// #[derive(Debug)]

/// SVC_DISCONNECT 2
#[derive(Debug)]
pub struct SvcDisconnect<'a> {
    pub reason: &'a [u8],
}

/// SVC_EVENT 3
#[derive(Debug)]
pub struct SvcEvent {
    // [bool; 5]
    pub event_count: BitType,
    pub events: Vec<EventS>,
}

#[derive(Debug)]
pub struct EventS {
    // [bool; 10]
    pub event_index: BitType,
    pub has_packet_index: bool,
    // [bool; 11]
    pub packet_index: Option<BitType>,
    pub has_delta: Option<bool>,
    pub delta: Option<Delta>,
    pub has_fire_time: bool,
    // [bool; 16]
    pub fire_time: Option<BitType>,
}

/// SVC_VERSION 4
#[derive(Debug)]
pub struct SvcVersion {
    pub protocol_version: u32,
}

/// SVC_SETVIEW 5
#[derive(Debug)]
pub struct SvcSetView {
    pub entity_index: i16,
}

/// SVC_SOUND 6
#[derive(Debug)]
pub struct SvcSound {
    // [bool; 9]
    pub flags: BitType,
    pub volume: Option<BitType>,
    pub attenuation: Option<BitType>,
    // [bool; 3]
    pub channel: BitType,
    // [bool; 11]
    pub entity_index: BitType,
    pub sound_index_long: Option<BitType>,
    pub sound_index_short: Option<BitType>,
    pub has_x: bool,
    pub has_y: bool,
    pub has_z: bool,
    // Very messed up.
    pub origin_x: f32,
    // Very messed up.
    pub origin_y: f32,
    // Very messed up.
    pub origin_z: f32,
    pub pitch: BitType,
}

#[derive(Debug)]
pub struct OriginCoord {
    pub int_flag: bool,
    pub fraction_flag: bool,
    pub is_negative: Option<bool>,
    pub int_value: Option<[bool; 12]>,
    pub fraction_value: Option<[bool; 3]>,
    pub unknown: [bool; 2],
}

/// SVC_TIME 7
#[derive(Debug)]
pub struct SvcTime {
    pub time: f32,
}

/// SVC_PRINT 8
#[derive(Debug)]
pub struct SvcPrint<'a> {
    pub message: &'a [u8],
}

/// SVC_STUFFTEXT 9
#[derive(Debug)]
pub struct SvcStuffText<'a> {
    pub command: &'a [u8],
}

/// SVC_SETANGLE 10
#[derive(Debug)]
pub struct SvcSetAngle {
    pub pitch: i16,
    pub yaw: i16,
    pub roll: i16,
}

/// SVC_SERVERINFO 11
#[derive(Debug)]
pub struct SvcServerInfo<'a> {
    pub protocol: i32,
    pub spawn_count: i32,
    pub map_checksum: i32,
    // [u8; 16]
    pub client_dll_hash: Vec<u8>,
    pub max_players: u8,
    pub player_index: u8,
    pub is_deathmatch: u8,
    pub game_dir: &'a [u8],
    pub hostname: &'a [u8],
    pub map_file_name: &'a [u8],
    pub map_cycle: &'a [u8],
    pub unknown: u8,
}

/// SVC_LIGHTSTYLE 12
#[derive(Debug)]
pub struct SvcLightStyle<'a> {
    pub index: u8,
    pub light_info: &'a [u8],
}

/// SVC_UPDATEUSERINFO 13
#[derive(Debug)]
pub struct SvcUpdateUserInfo<'a> {
    pub index: u8,
    pub id: u32,
    pub user_info: &'a [u8],
    // [u8; 16]
    pub cd_key_hash: &'a [u8],
}

/// SVC_DELTADESCRIPTION 14
#[derive(Debug)]
pub struct SvcDeltaDescription<'a> {
    pub name: &'a [u8],
    pub total_fields: u16,
    pub fields: DeltaDecoder<'a>,
}

/// SVC_CLIENTDATA 15
#[derive(Debug)]
pub struct SvcClientData {
    pub has_delta_update_mask: bool,
    // [bool; 8]
    pub delta_update_mask: Option<BitType>,
    pub client_data: Delta,
    pub has_weapon_data: bool,
    // [bool; 6]
    pub weapon_index: Option<BitType>,
    pub weapon_data: Option<Delta>,
}

/// SVC_STOPSOUND 16
#[derive(Debug)]
pub struct SvcStopSound {
    pub entity_index: i16,
}

/// SVC_PINGS 17
#[derive(Debug)]
pub struct SvcPings {
    pub pings: Vec<PingS>,
}

#[derive(Debug)]
pub struct PingS {
    pub has_ping_data: bool,
    pub player_id: Option<u8>,
    pub ping: Option<u8>,
    pub loss: Option<u8>,
}

/// SVC_PARTICLE 18
#[derive(Debug)]
pub struct SvcParticle {
    // Vec3
    pub origin: Vec<i16>,
    // Vec3
    pub direction: Vec<i8>,
    pub count: u8,
    pub color: u8,
}

/// SVC_PARTICLE 19

/// SVC_SPAWNSTATIC 20
#[derive(Debug)]
pub struct SvcSpawnStatic {
    pub model_index: i16,
    pub sequence: i8,
    pub frame: i8,
    pub color_map: i16,
    pub skin: i8,
    pub origin_x: i16,
    pub rotation_x: i8,
    pub origin_y: i16,
    pub rotation_y: i8,
    pub origin_z: i16,
    pub rotation_z: i8,
    pub has_render_mode: i8,
    // [u8; 3]
    pub render_color: Option<Vec<u8>>,
}

/// SVC_EVENTRELIABLE 21
#[derive(Debug)]
pub struct SvcEventReliable {
    // [bool; 10]
    pub event_index: BitType,
    pub event_args: Delta,
    pub has_fire_time: bool,
    // [bool; 16]
    pub fire_time: Option<BitType>,
}

/// SVC_SPAWNBASELINE 22
#[derive(Debug)]
pub struct SvcSpawnBaseline {
    pub entities: Vec<EntityS>,
}

#[derive(Debug)]
pub struct EntityS {
    // [bool; 11]
    pub index: BitType,
    // [bool; 2]
    pub type_: BitType,
    // One delta for 3 types
    pub delta: Delta,
    // [bool; 5]
    pub footer: BitType,
    // [bool; 6]
    pub total_extra_data: BitType,
    pub extra_data: Vec<Delta>,
}

/// SVC_TEMPENTITY 23
#[derive(Debug)]
pub struct SvcTempEntity<'a> {
    pub entity_type: u8,
    pub entity: TempEntityEntity<'a>,
}

#[repr(u8)]
#[derive(Debug)]
pub enum TempEntityEntity<'a> {
    // [u8; 24]
    TeBeamPoints(&'a [u8]) = 0,
    // [u8; 20]
    TeBeamEntPoint(&'a [u8]) = 1,
    // [u8; 6]
    TeGunshot(&'a [u8]) = 2,
    // [u8; 6]
    TeExplosion(&'a [u8]) = 3,
    // [u8; 6]
    TeTarExplosion(&'a [u8]) = 4,
    // [u8; 10]
    TeSmoke(&'a [u8]) = 5,
    // [u8; 12]
    TeTracer(&'a [u8]) = 6,
    // [u8; 17]
    TeLightning(&'a [u8]) = 7,
    // [u8; 16]
    TeBeamEnts(&'a [u8]) = 8,
    // [u8; 6]
    TeSparks(&'a [u8]) = 9,
    // [u8; 6]
    TeLavaSplash(&'a [u8]) = 10,
    // [u8; 6]
    TeTeleport(&'a [u8]) = 11,
    // [u8; 8]
    TeExplosion2(&'a [u8]) = 12,
    TeBspDecal(TeBspDecal<'a>) = 13,
    // [u8; 9]
    TeImplosion(&'a [u8]) = 14,
    // [u8; 19]
    TeSpriteRail(&'a [u8]) = 15,
    // [u8; 10]
    TeSprite(&'a [u8]) = 16,
    // [u8; 16]
    TeBeamSprite(&'a [u8]) = 18,
    // [u8; 24]
    TeBeamTorus(&'a [u8]) = 19,
    // [u8; 24]
    TeBeamDisk(&'a [u8]) = 20,
    // [u8; 24]
    TeBeamCylinder(&'a [u8]) = 21,
    // [u8; 10]
    TeBeamFollow(&'a [u8]) = 22,
    // [u8; 11]
    TeGlowSprite(&'a [u8]) = 23,
    // [u8; 16]
    TeBeamRing(&'a [u8]) = 24,
    // [u8; 19]
    TeStreakSplash(&'a [u8]) = 25,
    // [u8; 12]
    TeDLight(&'a [u8]) = 27,
    // [u8; 16]
    TeELight(&'a [u8]) = 28,
    TeTextMessage(TeTextMessage<'a>) = 29,
    // [u8; 17]
    TeLine(&'a [u8]) = 30,
    // [u8; 17]
    TeBox(&'a [u8]) = 31,
    // [u8; 2]
    TeKillBeam(&'a [u8]) = 99,
    // [u8; 10]
    TeLargeFunnel(&'a [u8]) = 100,
    // [u8; 14]
    TeBloodStream(&'a [u8]) = 101,
    // [u8; 12]
    TeShowLine(&'a [u8]) = 102,
    // [u8; 14]
    TeBlood(&'a [u8]) = 103,
    // [u8; 9]
    TeDecal(&'a [u8]) = 104,
    // [u8; 5]
    TeFizz(&'a [u8]) = 105,
    // [u8; 17]
    TeModel(&'a [u8]) = 106,
    // [u8; 13]
    TeExplodeModel(&'a [u8]) = 107,
    // [u8; 13]
    TeBreakModel(&'a [u8]) = 108,
    // [u8; 9]
    TeGunshotDecal(&'a [u8]) = 109,
    // [u8; 17]
    TeSpriteSpray(&'a [u8]) = 110,
    // [u8; 7]
    TeArmorRicochet(&'a [u8]) = 111,
    // [u8; 10]
    TePlayerDecal(&'a [u8]) = 112,
    // [u8; 10]
    TeBubbles(&'a [u8]) = 113,
    // [u8; 19]
    TeBubbleTrail(&'a [u8]) = 114,
    // [u8; 12]
    TeBloodSprite(&'a [u8]) = 115,
    // [u8; 7]
    TeWorldDecal(&'a [u8]) = 116,
    // [u8; 7]
    TeWorldDecalHigh(&'a [u8]) = 117,
    // [u8; 9]
    TeDecalHigh(&'a [u8]) = 118,
    // [u8; 16]
    TeProjectile(&'a [u8]) = 119,
    // [u8; 18]
    TeSpray(&'a [u8]) = 120,
    // [u8; 5]
    TePlayerSprites(&'a [u8]) = 121,
    // [u8; 10]
    TeParticleBurst(&'a [u8]) = 122,
    // [u8; 9]
    TeFireField(&'a [u8]) = 123,
    // [u8; 7]
    TePlayerAttachment(&'a [u8]) = 124,
    // [u8; 1]
    TeKillPlayerAttachment(&'a [u8]) = 125,
    // [u8; 10]
    TeMultigunShot(&'a [u8]) = 126,
    // [u8; 15]
    TeUserTracer(&'a [u8]) = 127,
}

#[derive(Debug)]
pub struct TeBspDecal<'a> {
    // [u8; 8]
    pub unknown1: &'a [u8],
    pub entity_index: i16,
    // [u8; 2]
    pub unknown2: Option<&'a [u8]>,
}

#[derive(Debug)]
pub struct TeTextMessage<'a> {
    pub channel: i8,
    pub x: i16,
    pub y: i16,
    pub effect: i8,
    // [u8; 4]
    pub text_color: &'a [u8],
    pub fade_in_time: i16,
    pub fade_out_time: i16,
    pub hold_time: i16,
    pub effect_time: Option<i16>,
    pub message: &'a [u8],
}

/// SVC_SETPAUSE 24
#[derive(Debug)]
pub struct SvcSetPause {
    pub is_paused: i8,
}

/// SVC_SIGNONNUM 25
#[derive(Debug)]
pub struct SvcSignOnNum {
    pub sign: i8,
}

/// SVC_CENTERPRINT 26
#[derive(Debug)]
pub struct SvcCenterPrint<'a> {
    pub message: &'a [u8],
}

/// SVC_KILLEDMONSTER 27
// #[derive(Debug)]

/// SVC_FOUNDSECRET 28
// #[derive(Debug)]

/// SVC_SPAWNSTATICSOUND 29
#[derive(Debug)]
pub struct SvcSpawnStaticSound {
    // Vec3
    pub origin: Vec<i16>,
    pub sound_index: u16,
    pub volume: u8,
    pub attenuation: u8,
    pub entity_index: u16,
    pub pitch: u8,
    pub flags: u8,
}

/// SVC_INTERMISSION 30
// #[derive(Debug)]

/// SVC_FINALE 31
#[derive(Debug)]
pub struct SvcFinale<'a> {
    pub text: &'a [u8],
}

/// SVC_CDTRACK 32
#[derive(Debug)]
pub struct SvcCdTrack {
    pub track: i8,
    pub loop_track: i8,
}

/// SVC_RESTORE 33
#[derive(Debug)]
pub struct SvcRestore<'a> {
    pub save_name: &'a [u8],
    pub map_count: u8,
    pub map_names: &'a [u8],
}

/// SVC_CUTSCENE 34
#[derive(Debug)]
pub struct SvcCutScene<'a> {
    pub text: &'a [u8],
}

/// SVC_WEAPONANIM 35
#[derive(Debug)]
pub struct SvcWeaponAnim {
    pub sequence_number: i8,
    pub weapon_model_body_group: i8,
}

/// SVC_DECALNAME 36
#[derive(Debug)]
pub struct SvcDecalname<'a> {
    pub position_index: u8,
    pub decal_name: &'a [u8],
}

/// SVC_ROOMTYPE 37
#[derive(Debug)]
pub struct SvcRoomType {
    pub room_type: u16,
}

/// SVC_ADDANGLE 38
#[derive(Debug)]
pub struct SvcAddAngle {
    pub angle_to_add: i16,
}

/// SVC_NEWUSERMSG 39
#[derive(Debug)]
pub struct SvcNewUserMsg<'a> {
    pub index: u8,
    pub size: u8,
    // [u8; 16]
    pub name: &'a [u8],
}

/// SVC_PACKETENTITIES (40)
#[derive(Debug)]
struct SvcPacketEntities {
    pub entity_count: u16,
    pub entity_states: Vec<EntityState>,
}

#[derive(Debug)]
pub struct EntityState {
    pub increment_entity_number: bool,
    pub is_absolute_entity_index: Option<bool>,
    pub absolute_entity_index: Option<[bool; 18]>,
    pub entity_index_difference: Option<[bool; 6]>,
    pub has_custom_delta: bool,
    pub has_baseline_index: bool,
    // TODO
    delta: u8,
}

/// SVC_DELTAPACKETENTITIES 41
#[derive(Debug)]
pub struct SvcDeltaPacketEntities {
    pub entity_count: u16,
    pub delta_sequence: u8,
    pub entity_states: Vec<EntityState>,
}

/// SVC_CHOKE 42

/// SVC_RESOURCELIST 43
#[derive(Debug)]
pub struct SvcResourceList<'a> {
    pub resource_count: [bool; 12],
    pub resources: Vec<Resource<'a>>,
    pub consistencies: Vec<Consistency>,
}

#[derive(Debug)]
pub struct Resource<'a> {
    pub type_: [bool; 4],
    pub name: &'a [u8],
    pub index: [bool; 12],
    pub size: [bool; 24],
    pub flag: [bool; 3],
    pub md5_hash: Option<[bool; 128]>,
    pub has_extra_info: bool,
    pub extra_info: Option<[bool; 256]>,
}

#[derive(Debug)]
pub struct Consistency {
    pub has_check_file_flag: bool,
    pub is_short_index: Option<bool>,
    pub short_index: Option<[bool; 5]>,
    pub long_index: Option<[bool; 10]>,
}

/// SVC_NEWMOVEVARS 44
#[derive(Debug)]
pub struct SvcNewMoveVars<'a> {
    pub gravity: f32,
    pub stop_speed: f32,
    pub max_speed: f32,
    pub spectator_max_speed: f32,
    pub accelerate: f32,
    pub airaccelerate: f32,
    pub water_accelerate: f32,
    pub friction: f32,
    pub edge_friction: f32,
    pub water_friction: f32,
    pub ent_garvity: f32,
    pub bounce: f32,
    pub step_size: f32,
    pub max_velocity: f32,
    pub z_max: f32,
    pub wave_height: f32,
    pub footsteps: i32,
    pub roll_angle: f32,
    pub roll_speed: f32,
    // Vec3
    pub sky_color: Vec<f32>,
    // Vec3
    pub sky_vec: Vec<f32>,
    pub sky_name: &'a [u8],
}

/// SVC_RESOURCEREQUEST 45
#[derive(Debug)]
pub struct SvcResourceRequest {
    pub spawn_count: i32,
    pub unknown: u32,
}

/// SVC_CUSTOMIZATION 46
#[derive(Debug)]
pub struct SvcCustomization<'a> {
    pub player_index: u8,
    pub type_: u8,
    pub name: &'a [u8],
    pub index: u16,
    pub download_size: u32,
    pub flags: u8,
    pub md5_hash: Option<[u8; 16]>,
}

/// SVC_CROSSHAIRANGLE 47
#[derive(Debug)]
pub struct SvcCrosshairAngle {
    pub pitch: i16,
    pub yaw: i16,
}

/// SVC_SOUNDFADE 48
#[derive(Debug)]
pub struct SvcSoundFade {
    pub initial_percent: u8,
    pub hold_time: u8,
    pub fade_out_time: u8,
    pub fade_in_time: u8,
}

/// SVC_FILETXFERFAILED 49
#[derive(Debug)]
pub struct SvcFileTxferFailed<'a> {
    pub file_name: &'a [u8],
}

/// SVC_HLTV 50
#[derive(Debug)]
pub struct SvcHltv {
    pub mode: u8,
}

/// SVC_DIRECTOR 51
#[derive(Debug)]
pub struct SvcDirector<'a> {
    pub length: u8,
    pub flag: u8,
    pub message: &'a [u8],
}

/// SVC_VOINCEINIT 52
#[derive(Debug)]
pub struct SvcVoiceInit<'a> {
    pub codec_name: &'a [u8],
    pub quality: i8,
}

/// SVC_VOICEDATA 53
#[derive(Debug)]
pub struct SvcVoiceData<'a> {
    pub player_index: u8,
    pub size: u16,
    pub data: &'a [u8],
}

/// SVC_SENDEXTRAINFO 54
#[derive(Debug)]
pub struct SvcSendExtraInfo<'a> {
    pub fallback_dir: &'a [u8],
    pub can_cheat: u8,
}

/// SVC_TIMESCALE 55
#[derive(Debug)]
pub struct SvcTimeScale {
    pub time_scale: f32,
}

/// SVC_RESOURCELOCATION 56
#[derive(Debug)]
pub struct SvcResourceLocation<'a> {
    pub download_url: &'a [u8],
}

/// SVC_SENDCVARVALUE 57
#[derive(Debug)]
pub struct SvcSendCvarValue<'a> {
    pub name: &'a [u8],
}

/// SVC_SENDCVARVALUE2 58
#[derive(Debug)]
pub struct SvcSendCvarValue2<'a> {
    pub request_id: u32,
    name: &'a [u8],
}

#[derive(Debug)]
pub enum Message<'a> {
    UserMessage(NetMsgUserMessage<'a>),
    EngineMessage(EngineMessage<'a>),
}

pub enum MessageType {
    UserMessage,
    EngineMessageType(EngineMessageType),
}

#[repr(u8)]
#[derive(Debug)]
pub enum EngineMessage<'a> {
    SvcBad = 0,
    SvcNop = 1,
    SvcDisconnect(SvcDisconnect<'a>) = 2,
    SvcEvent(SvcEvent) = 3,
    SvcVersion(SvcVersion) = 4,
    SvcSetView(SvcSetView) = 5,
    SvcSound(SvcSound) = 6,
    SvcTime(SvcTime) = 7,
    SvcPrint(SvcPrint<'a>) = 8,
    SvcStuffText(SvcStuffText<'a>) = 9,
    SvcSetAngle(SvcSetAngle) = 10,
    SvcServerInfo(SvcServerInfo<'a>) = 11,
    SvcLightStyle(SvcLightStyle<'a>) = 12,
    SvcUpdateuserInfo(SvcUpdateUserInfo<'a>) = 13,
    SvcDeltaDescription(SvcDeltaDescription<'a>) = 14,
    SvcClientData(SvcClientData) = 15,
    SvcStopSound(SvcStopSound) = 16,
    SvcPings(SvcPings) = 17,
    SvcParticle(SvcParticle) = 18,
    SvcDamage = 19,
    SvcSpawnStatic(SvcSpawnStatic) = 20,
    SvcEventReliable(SvcEventReliable) = 21,
    SvcSpawnBaseline(SvcSpawnBaseline) = 22,
    SvcTempEntity(SvcTempEntity<'a>) = 23,
    SvcSetPause(SvcSetPause) = 24,
    SvcSignOnNum(SvcSignOnNum) = 25,
    SvcCenterPrint(SvcCenterPrint<'a>) = 26,
    SvcKilledMonster = 27,
    SvcFoundSecret = 28,
    SvcSpawnStaticSound = 29,
    SvcIntermission = 30,
    SvcFinale = 31,
    SvcCdTrack(SvcCdTrack) = 32,
    SvcRestore = 33,
    SvcCutscene = 34,
    SvcWeaponAnim = 35,
    SvcDecalName = 36,
    SvcRoomType = 37,
    SvcAddAngle = 38,
    SvcNewUserMsg(SvcNewUserMsg<'a>) = 39,
    SvcPacketEntities = 40,
    SvcDeltaPacketEntities = 41,
    SvcChoke = 42,
    SvcResourceList = 43,
    SvcNewMoveVars(SvcNewMoveVars<'a>) = 44,
    SvcResourceRequest = 45,
    SvcCustomization = 46,
    SvcCrosshairAngle = 47,
    SvcSoundFade = 48,
    SvcFileTxferFailed = 49,
    SvcHltv = 50,
    SvcDirector = 51,
    SvcVoiceInit = 52,
    SvcVoiceData = 53,
    SvcSendExtraInfo(SvcSendExtraInfo<'a>) = 54,
    SvcTimeScale = 55,
    SvcResourceLocation = 56,
    SvcSendCvarValue = 57,
    SvcSendCvarValue2 = 58,
}

// Eh, yes.
#[repr(u8)]
pub enum EngineMessageType {
    SvcBad = 0,
    SvcNop = 1,
    SvcDisconnect = 2,
    SvcEvent = 3,
    SvcVersion = 4,
    SvcSetView = 5,
    SvcSound = 6,
    SvcTime = 7,
    SvcPrint = 8,
    SvcStuffText = 9,
    SvcSetAngle = 10,
    SvcServerInfo = 11,
    SvcLightStyle = 12,
    SvcUpdateUserInfo = 13,
    SvcDeltaDescription = 14,
    SvcClientData = 15,
    SvcStopSound = 16,
    SvcPings = 17,
    SvcParticle = 18,
    SvcDamage = 19,
    SvcSpawnStatic = 20,
    SvcEventReliable = 21,
    SvcSpawnBaseline = 22,
    SvcTempEntity = 23,
    SvcSetPause = 24,
    SvcSignOnNum = 25,
    SvcCenterPrint = 26,
    SvcKilledMonster = 27,
    SvcFoundSecret = 28,
    SvcSpawnStaticSound = 29,
    SvcIntermission = 30,
    SvcFinale = 31,
    SvcCdTrack = 32,
    SvcRestore = 33,
    SvcCutscene = 34,
    SvcWeaponAnim = 35,
    SvcDecalName = 36,
    SvcRoomType = 37,
    SvcAddAngle = 38,
    SvcNewUserMsg = 39,
    SvcPacketEntities = 40,
    SvcDeltaPacketEntities = 41,
    SvcChoke = 42,
    SvcResourceList = 43,
    SvcNewMoveVars = 44,
    SvcResourceRequest = 45,
    SvcCustomization = 46,
    SvcCrosshairAngle = 47,
    SvcSoundFade = 48,
    SvcFileTxferFailed = 49,
    SvcHltv = 50,
    SvcDirector = 51,
    SvcVoiceInit = 52,
    SvcVoiceData = 53,
    SvcSendExtraInfo = 54,
    SvcTimeScale = 55,
    SvcResourceLocation = 56,
    SvcSendCvarValue = 57,
    SvcSendCvarValue2 = 58,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::EngineMessageType(EngineMessageType::SvcBad),
            1 => MessageType::EngineMessageType(EngineMessageType::SvcNop),
            2 => MessageType::EngineMessageType(EngineMessageType::SvcDisconnect),
            3 => MessageType::EngineMessageType(EngineMessageType::SvcEvent),
            4 => MessageType::EngineMessageType(EngineMessageType::SvcVersion),
            5 => MessageType::EngineMessageType(EngineMessageType::SvcSetView),
            6 => MessageType::EngineMessageType(EngineMessageType::SvcSound),
            7 => MessageType::EngineMessageType(EngineMessageType::SvcTime),
            8 => MessageType::EngineMessageType(EngineMessageType::SvcPrint),
            9 => MessageType::EngineMessageType(EngineMessageType::SvcStuffText),
            10 => MessageType::EngineMessageType(EngineMessageType::SvcSetAngle),
            11 => MessageType::EngineMessageType(EngineMessageType::SvcServerInfo),
            12 => MessageType::EngineMessageType(EngineMessageType::SvcLightStyle),
            13 => MessageType::EngineMessageType(EngineMessageType::SvcUpdateUserInfo),
            14 => MessageType::EngineMessageType(EngineMessageType::SvcDeltaDescription),
            15 => MessageType::EngineMessageType(EngineMessageType::SvcClientData),
            16 => MessageType::EngineMessageType(EngineMessageType::SvcStopSound),
            17 => MessageType::EngineMessageType(EngineMessageType::SvcPings),
            18 => MessageType::EngineMessageType(EngineMessageType::SvcParticle),
            19 => MessageType::EngineMessageType(EngineMessageType::SvcDamage),
            20 => MessageType::EngineMessageType(EngineMessageType::SvcSpawnStatic),
            21 => MessageType::EngineMessageType(EngineMessageType::SvcEventReliable),
            22 => MessageType::EngineMessageType(EngineMessageType::SvcSpawnBaseline),
            23 => MessageType::EngineMessageType(EngineMessageType::SvcTempEntity),
            24 => MessageType::EngineMessageType(EngineMessageType::SvcSetPause),
            25 => MessageType::EngineMessageType(EngineMessageType::SvcSignOnNum),
            26 => MessageType::EngineMessageType(EngineMessageType::SvcCenterPrint),
            27 => MessageType::EngineMessageType(EngineMessageType::SvcKilledMonster),
            28 => MessageType::EngineMessageType(EngineMessageType::SvcFoundSecret),
            29 => MessageType::EngineMessageType(EngineMessageType::SvcSpawnStaticSound),
            30 => MessageType::EngineMessageType(EngineMessageType::SvcIntermission),
            31 => MessageType::EngineMessageType(EngineMessageType::SvcFinale),
            32 => MessageType::EngineMessageType(EngineMessageType::SvcCdTrack),
            33 => MessageType::EngineMessageType(EngineMessageType::SvcRestore),
            34 => MessageType::EngineMessageType(EngineMessageType::SvcCutscene),
            35 => MessageType::EngineMessageType(EngineMessageType::SvcWeaponAnim),
            36 => MessageType::EngineMessageType(EngineMessageType::SvcDecalName),
            37 => MessageType::EngineMessageType(EngineMessageType::SvcRoomType),
            38 => MessageType::EngineMessageType(EngineMessageType::SvcAddAngle),
            39 => MessageType::EngineMessageType(EngineMessageType::SvcNewUserMsg),
            40 => MessageType::EngineMessageType(EngineMessageType::SvcPacketEntities),
            41 => MessageType::EngineMessageType(EngineMessageType::SvcDeltaPacketEntities),
            42 => MessageType::EngineMessageType(EngineMessageType::SvcChoke),
            43 => MessageType::EngineMessageType(EngineMessageType::SvcResourceList),
            44 => MessageType::EngineMessageType(EngineMessageType::SvcNewMoveVars),
            45 => MessageType::EngineMessageType(EngineMessageType::SvcResourceRequest),
            46 => MessageType::EngineMessageType(EngineMessageType::SvcCustomization),
            47 => MessageType::EngineMessageType(EngineMessageType::SvcCrosshairAngle),
            48 => MessageType::EngineMessageType(EngineMessageType::SvcSoundFade),
            49 => MessageType::EngineMessageType(EngineMessageType::SvcFileTxferFailed),
            50 => MessageType::EngineMessageType(EngineMessageType::SvcHltv),
            51 => MessageType::EngineMessageType(EngineMessageType::SvcDirector),
            52 => MessageType::EngineMessageType(EngineMessageType::SvcVoiceInit),
            53 => MessageType::EngineMessageType(EngineMessageType::SvcVoiceData),
            54 => MessageType::EngineMessageType(EngineMessageType::SvcSendExtraInfo),
            55 => MessageType::EngineMessageType(EngineMessageType::SvcTimeScale),
            56 => MessageType::EngineMessageType(EngineMessageType::SvcResourceLocation),
            57 => MessageType::EngineMessageType(EngineMessageType::SvcSendCvarValue),
            58 => MessageType::EngineMessageType(EngineMessageType::SvcSendCvarValue2),
            _ => MessageType::UserMessage,
        }
    }
}
