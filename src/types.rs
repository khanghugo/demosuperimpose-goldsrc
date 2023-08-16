// pub struct NetMsgMessageBlock<'a> {
//     pub type_: NetMsgMessageData,
//     pub data: NetMsgMessageData,
// }

use std::{collections::HashMap, ops::BitAnd};

use bitvec::{slice::BitSlice, vec::BitVec};

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

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

// UserMessage
pub struct NetMsgUserMessage<'a> {
    pub message: &'a [u8],
}

/// SVC_BAD 0

/// SVC_NOP 1

/// SVC_DISCONNECT 2
pub struct SvcDisconnect<'a> {
    pub reason: &'a [u8],
}

/// SVC_EVENT 3
pub struct SvcEvent<'a> {
    /// [bool; 5]
    pub event_count: Vec<bool>,
    pub events: Vec<EventS<'a>>,
}

pub struct EventS<'a> {
    /// [bool; 10]
    pub event_index: BitType,
    pub has_packet_index: bool,
    /// [bool; 11]
    pub packet_index: Option<BitType>,
    pub has_delta: Option<bool>,
    // TODO
    pub delta: DeltaDecoder<'a>,
    pub has_fire_time: bool,
    pub fire_time: [bool; 16],
}

/// SVC_VERSION 4
pub struct SvcVersion {
    protocol_version: u32,
}

/// SVC_SETVIEW 5
pub struct SvcSetView {
    entity_index: i16,
}

/// SVC_SOUND 6
pub struct SvcSound {
    pub flags: [bool; 9],
    pub volume: Option<u8>,
    pub attenuation: Option<u8>,
    pub channel: [bool; 3],
    pub entity_index: [bool; 11],
    pub sound_index_long: Option<u16>,
    pub sound_index_short: Option<u8>,
    pub has_x: bool,
    pub has_y: bool,
    pub has_z: bool,
    pub origin_x: Option<OriginCoord>,
    pub origin_y: Option<OriginCoord>,
    pub origin_z: Option<OriginCoord>,
    pub pitch: u8,
}

pub struct OriginCoord {
    pub int_flag: bool,
    pub fraction_flag: bool,
    pub is_negative: Option<bool>,
    pub int_value: Option<[bool; 12]>,
    pub fraction_value: Option<[bool; 3]>,
    pub unknown: [bool; 2],
}

/// SVC_TIME 7
pub struct SvcTime {
    pub time: f32,
}

/// SVC_PRINT 8
pub struct SvcPrint<'a> {
    pub message: &'a [u8],
}

/// SVC_STUFFTEXT 9
pub struct SvcStuffText<'a> {
    pub command: &'a [u8],
}

/// SVC_SETANGLE 10
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
pub struct SvcLightStyle<'a> {
    pub index: u8,
    pub light_info: &'a [u8],
}

/// SVC_UPDATEUSERINFO 13
pub struct SvcUpdateUserInfo<'a> {
    pub index: u8,
    pub id: u32,
    pub user_info: &'a [u8],
    pub cd_key_hash: [u8; 16],
}

/// SVC_DELTADESCRIPTION 14
pub struct SvcDeltaDescription<'a> {
    pub name: &'a [u8],
    pub total_fields: u16,
    pub fields: Vec<DeltaDecoder<'a>>,
}

/// SVC_CLIENTDATA 15
#[derive(Debug)]
pub struct SvcClientData {
    pub has_delta_update_mask: bool,
    /// [bool; 8]
    pub delta_update_mask: Option<BitType>,
    pub client_data: Delta,
    pub has_weapon_data: bool,
    /// [bool; 6]
    pub weapon_index: Option<BitType>,
    pub weapon_data: Option<Delta>,
}

/// SVC_STOPSOUND 16
pub struct SvcStopSound {
    pub entity_index: i16,
}

/// SVC_PINGS 17
pub struct SvcPings {
    pub pings: Vec<Ping>,
}

pub struct Ping {
    pub has_ping_data: bool,
    pub player_id: Option<u8>,
    pub ping: Option<u8>,
    pub loss: Option<u8>,
}

/// SVC_PARTICLE 18
pub struct SvcParticle {
    pub origin: Vec3<i16>,
    pub direction: Vec3<i8>,
    pub count: u8,
    pub color: u8,
}

/// SVC_PARTICLE 19

/// SVC_SPAWNSTATIC 20
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
    pub render_color: Option<[u8; 3]>,
}

/// SVC_EVENTRELIABLE 21
pub struct SvcEventReliable {
    pub event_index: [bool; 10],
    // TODO
    pub rest: u8,
}

/// SVC_SPAWNBASELINE 22
pub struct SvcSpawnBaseline {
    pub entities: Vec<Entity>,
}

pub struct Entity {
    pub index: [bool; 110],
    pub type_: [bool; 2],
    // TODO
    pub rest: u8,
}

/// SVC_TEMPENTITY 23
pub struct SvcTempEntity<'a> {
    entity_type: u8,
    entity: TempEntityEntity<'a>,
}

#[repr(u8)]
pub enum TempEntityEntity<'a> {
    TeBeamPoints([u8; 24]) = 0,
    TeBeamEntPoint([u8; 20]) = 1,
    TeGunshot([u8; 6]) = 2,
    TeExplosion([u8; 6]) = 3,
    TeTarExplosion([u8; 6]) = 4,
    TeSmoke([u8; 10]) = 5,
    TeTracer([u8; 12]) = 6,
    TeLightning([u8; 17]) = 7,
    TeBeamEnts([u8; 16]) = 8,
    TeSparks([u8; 6]) = 9,
    TeLavaSplash([u8; 6]) = 10,
    TeTeleport([u8; 6]) = 11,
    TeExplosion2([u8; 8]) = 12,
    TeBspDecal(TeBspDecal) = 13,
    TeImplosion([u8; 9]) = 14,
    TeSpriteRail([u8; 19]) = 15,
    TeSprite([u8; 10]) = 16,
    TeBeamSprite([u8; 16]) = 18,
    TeBeamTorus([u8; 24]) = 19,
    TeBeamDisk([u8; 24]) = 20,
    TeBeamCylinder([u8; 24]) = 21,
    TeBeamFollow([u8; 10]) = 22,
    TeGlowSprite([u8; 11]) = 23,
    TeBeamRing([u8; 16]) = 24,
    TeStreakSplash([u8; 19]) = 25,
    TeDLight([u8; 12]) = 27,
    TeELight([u8; 16]) = 28,
    TeTextMessage(TeTextMessage<'a>) = 29,
    TeLine([u8; 17]) = 30,
    TeBox([u8; 17]) = 31,
    TeKillBeam([u8; 2]) = 99,
    TeLargeFunnel([u8; 10]) = 100,
    TeBloodStream([u8; 14]) = 101,
    TeShowLine([u8; 12]) = 102,
    TeBlood([u8; 14]) = 103,
    TeDecal([u8; 9]) = 104,
    TeFizz([u8; 5]) = 105,
    TeModel([u8; 17]) = 106,
    TeExplodeModel([u8; 13]) = 107,
    TeBreakModel([u8; 13]) = 108,
    TeGunshotDecal([u8; 9]) = 109,
    TeSpriteSpray([u8; 17]) = 110,
    TeArmorRicochet([u8; 7]) = 111,
    TePlayerDecal([u8; 10]) = 112,
    TeBubbles([u8; 10]) = 113,
    TeBubbleTrail([u8; 19]) = 114,
    TeBloodSprite([u8; 12]) = 115,
    TeWorldDecal([u8; 7]) = 116,
    TeWorldDecalHigh([u8; 7]) = 117,
    TeDecalHigh([u8; 9]) = 118,
    TeProjectile([u8; 16]) = 119,
    TeSpray([u8; 18]) = 120,
    TePlayerSprites([u8; 5]) = 121,
    TeParticleBurst([u8; 10]) = 122,
    TeFireField([u8; 9]) = 123,
    TePlayerAttachment([u8; 7]) = 124,
    TeKillPlayerAttachment([u8; 1]) = 125,
    TeMultigunShot([u8; 10]) = 126,
    TeUserTracer([u8; 15]) = 127,
}

pub struct TeBspDecal {
    pub unknown1: [u8; 8],
    pub entity_index: i16,
    pub unknown2: Option<[u8; 2]>,
}

pub struct TeTextMessage<'a> {
    pub channel: i8,
    pub x: i16,
    pub y: i16,
    pub effect: i8,
    pub text_color: [u8; 4],
    pub fade_in_time: i16,
    pub fade_out_time: i16,
    pub hold_time: i16,
    pub effect_time: Option<i16>,
    pub mesage: &'a [u8],
}

/// SVC_SETPAUSE 24
pub struct SvcSetPause {
    pub is_paused: u8,
}

/// SVC_SIGNONNUM 25
pub struct SvcSigonNum {
    sign: u8,
}

/// SVC_CENTERPRINT 26
pub struct SvcCenterPrint<'a> {
    message: &'a [u8],
}

/// SVC_KILLEDMONSTER 27

/// SVC_FOUNDSECRET 28

/// SVC_SPAWNSTATICSOUND 29
pub struct SvcSpawnStaticSound {
    pub origin: Vec3<i16>,
    pub sound_index: u16,
    pub volume: u8,
    pub attenuation: u8,
    pub entity_index: u16,
    pub pitch: u8,
    pub flags: u8,
}

/// SVC_INTERMISSION 30

/// SVC_FINALE 31
pub struct SvcFinale<'a> {
    pub text: &'a [u8],
}

/// SVC_CDTRACK 32
pub struct SvcCdTrack {
    pub track: i8,
    pub loop_track: i8,
}

/// SVC_RESTORE 33
pub struct SvcRestore<'a> {
    pub save_name: &'a [u8],
    pub map_count: u8,
    pub map_names: &'a [u8],
}

/// SVC_CUTSCENE 34
pub struct SvcCutScene<'a> {
    pub text: &'a [u8],
}

/// SVC_WEAPONANIM 35
pub struct SvcWeaponAnim {
    pub sequence_number: i8,
    pub weapon_model_body_group: i8,
}

/// SVC_DECALNAME 36
pub struct SvcDecalname<'a> {
    pub position_index: u8,
    pub decal_name: &'a [u8],
}

/// SVC_ROOMTYPE 37
pub struct SvcRoomType {
    pub room_type: u16,
}

/// SVC_ADDANGLE 38
pub struct SvcAddAngle {
    pub angle_to_add: i16,
}

/// SVC_NEWUSERMSG 39
pub struct SvcNewUserMsg {
    pub index: u8,
    pub size: u8,
    pub name: [u8; 16],
}

/// SVC_PACKETENTITIES (40)
struct SvcPacketEntities {
    pub entity_count: u16,
    pub entity_states: Vec<EntityState>,
}

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
pub struct SvcDeltaPacketEntities {
    pub entity_count: u16,
    pub delta_sequence: u8,
    pub entity_states: Vec<EntityState>,
}

/// SVC_CHOKE 42

/// SVC_RESOURCELIST 43
pub struct SvcResourceList<'a> {
    pub resource_count: [bool; 12],
    pub resources: Vec<Resource<'a>>,
    pub consistencies: Vec<Consistency>,
}

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

pub struct Consistency {
    pub has_check_file_flag: bool,
    pub is_short_index: Option<bool>,
    pub short_index: Option<[bool; 5]>,
    pub long_index: Option<[bool; 10]>,
}

/// SVC_NEWMOVEVARS 44
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
    pub sky_color: Vec3<f32>,
    pub sky_vec: Vec3<f32>,
    pub sky_name: &'a [u8],
}

/// SVC_RESOURCEREQUEST 45
pub struct SvcResourceRequest {
    pub spawn_count: i32,
    pub unknown: u32,
}

/// SVC_CUSTOMIZATION 46
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
pub struct SvcCrosshairAngle {
    pub pitch: i16,
    pub yaw: i16,
}

/// SVC_SOUNDFADE 48
pub struct SvcSoundFade {
    pub initial_percent: u8,
    pub hold_time: u8,
    pub fade_out_time: u8,
    pub fade_in_time: u8,
}

/// SVC_FILETXFERFAILED 49
pub struct SvcFileTxferFailed<'a> {
    pub file_name: &'a [u8],
}

/// SVC_HLTV 50
pub struct SvcHltv {
    pub mode: u8,
}

/// SVC_DIRECTOR 51
pub struct SvcDirector<'a> {
    pub length: u8,
    pub flag: u8,
    pub message: &'a [u8],
}

/// SVC_VOINCEINIT 52
pub struct SvcVoiceInit<'a> {
    pub codec_name: &'a [u8],
    pub quality: i8,
}

/// SVC_VOICEDATA 53
pub struct SvcVoiceData<'a> {
    pub player_index: u8,
    pub size: u16,
    pub data: &'a [u8],
}

/// SVC_SENDEXTRAINFO 54
pub struct SvcSendExtraInfo<'a> {
    pub fallback_dir: &'a [u8],
    pub can_cheat: u8,
}

/// SVC_TIMESCALE 55
pub struct SvcTimeScale {
    pub time_scale: f32,
}

/// SVC_RESOURCELOCATION 56
pub struct SvcResourceLocation<'a> {
    pub download_url: &'a [u8],
}

/// SVC_SENDCVARVALUE 57
pub struct SvcSendCvarValue<'a> {
    pub name: &'a [u8],
}

/// SVC_SENDCVARVALUE2 58
pub struct SvcSendCvarValue2<'a> {
    pub request_id: u32,
    name: &'a [u8],
}

pub enum NetMsgMessageBlock {
    UserMessage,
    EngineMessage(EngineMessageType),
}

#[repr(u8)]
pub enum EngineMessageType {
    SvcBad = 0,
    SvcNop = 1,
    SvcDisconnect = 2,
    SvcEvent = 3,
    SvcVersion = 4,
    SvcSetview = 5,
    SvcSound = 6,
    SvcTime = 7,
    SvcPrint = 8,
    SvcStuffText = 9,
    SvcSetAngle = 10,
    SvcServerInfo = 11,
    SvcLightStyle = 12,
    SvcUpdateuserInfo = 13,
    SvcDeltaDescription = 14,
    SvcClientData = 15,
    SvcStopsound = 16,
    SvcPings = 17,
    SvcParticle = 18,
    SvcDamage = 19,
    SvcSpawnStatic = 20,
    SvcEvenReliable = 21,
    SvcSpawnBaseline = 22,
    SvcTempEntity = 23,
    SvcSetPause = 24,
    SvcSignonNum = 25,
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
