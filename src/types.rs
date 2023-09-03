use std::collections::HashMap;

use bitvec::vec::BitVec;

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

// pub struct delta_description_s {
//     pub field_type: i32,
//     pub field_name: [u8; 32],
//     pub field_offset: i32,
//     pub field_size: i16,
//     pub significant_bits: i32,
//     pub premultiply: f32,
//     pub postmultiply: f32,
//     pub flags: i16,
//     pub stats: delta_stats_t,
// }

// pub struct delta_stats_t {
//     pub send_count: i32,
//     pub received_count: i32,
// }

/// A simplified struct of delta_description_s
///
/// Lots of info end up unused.
#[derive(Clone, Debug)]
pub struct DeltaDecoderS {
    pub name: Vec<u8>,
    pub bits: u32,
    pub divisor: f32,
    pub flags: u32,
    // Quick and dirty way to do delta write.
    // Engine does this I think but with flags member.
    pub should_write: bool,
}

pub type Delta = HashMap<String, Vec<u8>>;
pub type DeltaDecoder = Vec<DeltaDecoderS>;
pub type DeltaDecoderTable = HashMap<String, DeltaDecoder>;

pub type BitType = BitVec<u8>;

// pub struct delta_description_t<'a> {
//     flags: u32,
//     name:
// }

/// UserMessage
#[derive(Clone, Debug)]
pub struct NetMsgUserMessage<'a> {
    pub id: u8,
    // [bool; 16]
    pub name: &'a [u8],
    pub data: &'a [u8],
}

/// SVC_BAD 0
// #[derive(Clone, Debug)]

/// SVC_NOP 1
// #[derive(Clone, Debug)]

/// SVC_DISCONNECT 2
#[derive(Clone, Debug)]
pub struct SvcDisconnect<'a> {
    pub reason: &'a [u8],
}

/// SVC_EVENT 3
#[derive(Clone, Debug)]
pub struct SvcEvent {
    // [bool; 5]
    pub event_count: BitType,
    pub events: Vec<EventS>,
}

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct SvcVersion {
    pub protocol_version: u32,
}

/// SVC_SETVIEW 5
#[derive(Clone, Debug)]
pub struct SvcSetView {
    pub entity_index: i16,
}

/// SVC_SOUND 6
#[derive(Clone, Debug)]
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
    pub origin_x: Option<OriginCoord>,
    pub origin_y: Option<OriginCoord>,
    pub origin_z: Option<OriginCoord>,
    pub pitch: BitType,
}

#[derive(Clone, Debug)]
pub struct OriginCoord {
    pub int_flag: bool,
    pub fraction_flag: bool,
    pub is_negative: Option<bool>,
    // [bool; 12]
    pub int_value: Option<BitType>,
    // [bool; 3]
    pub fraction_value: Option<BitType>,
    // There is no unknow, Xd
    // [bool; 2]
    // pub unknown: BitType,
}

/// SVC_TIME 7
#[derive(Clone, Debug)]
pub struct SvcTime {
    pub time: f32,
}

/// SVC_PRINT 8
#[derive(Clone, Debug)]
pub struct SvcPrint<'a> {
    pub message: &'a [u8],
}

/// SVC_STUFFTEXT 9
#[derive(Clone, Debug)]
pub struct SvcStuffText<'a> {
    pub command: &'a [u8],
}

/// SVC_SETANGLE 10
#[derive(Clone, Debug)]
pub struct SvcSetAngle {
    pub pitch: i16,
    pub yaw: i16,
    pub roll: i16,
}

/// SVC_SERVERINFO 11
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct SvcLightStyle<'a> {
    pub index: u8,
    pub light_info: &'a [u8],
}

/// SVC_UPDATEUSERINFO 13
#[derive(Clone, Debug)]
pub struct SvcUpdateUserInfo<'a> {
    pub index: u8,
    pub id: u32,
    pub user_info: &'a [u8],
    // [u8; 16]
    pub cd_key_hash: &'a [u8],
}

/// SVC_DELTADESCRIPTION 14
#[derive(Clone, Debug)]
pub struct SvcDeltaDescription<'a> {
    pub name: &'a [u8],
    pub total_fields: u16,
    pub fields: DeltaDecoder,
    pub clone: Vec<u8>,
}

/// SVC_CLIENTDATA 15
#[derive(Clone, Debug)]
pub struct SvcClientData {
    pub has_delta_update_mask: bool,
    // [bool; 8]
    pub delta_update_mask: Option<BitType>,
    pub client_data: Delta,
    pub weapon_data: Option<Vec<ClientDataWeaponData>>,
    pub clone: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct ClientDataWeaponData {
    // [bool; 6]
    pub weapon_index: BitType,
    pub weapon_data: Delta,
}

/// SVC_STOPSOUND 16
#[derive(Clone, Debug)]
pub struct SvcStopSound {
    pub entity_index: i16,
}

/// SVC_PINGS 17
#[derive(Clone, Debug)]
pub struct SvcPings {
    pub pings: Vec<PingS>,
}

#[derive(Clone, Debug)]
pub struct PingS {
    pub has_ping_data: bool,
    pub player_id: Option<u8>,
    pub ping: Option<u8>,
    pub loss: Option<u8>,
}

/// SVC_PARTICLE 18
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct SvcEventReliable {
    // [bool; 10]
    pub event_index: BitType,
    pub event_args: Delta,
    pub has_fire_time: bool,
    // [bool; 16]
    pub fire_time: Option<BitType>,
}

/// SVC_SPAWNBASELINE 22
#[derive(Clone, Debug)]
pub struct SvcSpawnBaseline {
    pub entities: Vec<EntityS>,
    // These members are not inside EntityS like cgdangelo/talent suggests.
    // [bool; 6]
    pub total_extra_data: BitType,
    pub extra_data: Vec<Delta>,
}

#[derive(Clone, Debug)]
pub struct EntityS {
    // [bool; 11]
    pub index: BitType,
    // [bool; 2]
    pub type_: BitType,
    // One delta for 3 types
    pub delta: Delta,
}

/// SVC_TEMPENTITY 23
#[derive(Clone, Debug)]
pub struct SvcTempEntity<'a> {
    pub entity_type: u8,
    pub entity: TempEntityEntity<'a>,
}

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum TempEntityEntity<'a> {
    // [u8; 24]
    TeBeamPoints(TeBeamPoints<'a>) = 0,
    // [u8; 20]
    TeBeamEntPoint(&'a [u8]) = 1,
    // [u8; 6]
    TeGunshot(&'a [u8]) = 2,
    // It is 11
    // [u8; 11]
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
    TeSpriteTrail(&'a [u8]) = 15,
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
    // It is 24
    // [u8; 24]
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
    // It is 18.
    // [u8; 18]
    TeMultigunShot(&'a [u8]) = 126,
    // [u8; 15]
    TeUserTracer(&'a [u8]) = 127,
}

// TE_BEAMPOINTS 0
#[derive(Clone, Debug)]
pub struct TeBeamPoints<'a> {
    // [i16; 3]
    pub start_position: Vec<i16>,
    // [i16; 3]
    pub end_position: Vec<i16>,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [u8; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_BEAMENTPOINTS 1
#[derive(Clone, Debug)]
pub struct TeBeamEntPoint<'a> {
    pub start_entity: i16,
    // [i16; 3]
    pub end_position: &'a [i16],
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_GUNSHOT 2
#[derive(Clone, Debug)]
pub struct TeGunShot<'a> {
    // [i16; 3]
    pub position: &'a [i16],
}

// TE_EXPLOSION 3
#[derive(Clone, Debug)]
pub struct TeExplosion<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub sprite_index: i16,
    pub scale: u8,
    pub frame_rame: u8,
    pub flags: u8,
}

// TE_TAREXPLOSION 4
#[derive(Clone, Debug)]
pub struct TeTarExplosion<'a> {
    // [i16; 3]
    pub position: &'a [i16],
}

// TE_SMOKE 5
#[derive(Clone, Debug)]
pub struct TeSmoke<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub sprite_index: i16,
    pub scale: u8,
    pub frame_rate: u8,
}

// TE_TRACER 6
#[derive(Clone, Debug)]
pub struct TeTracer<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub end_position: &'a [i16],
}

// TE_LIGHTNING 7
#[derive(Clone, Debug)]
pub struct TeLightning<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub end_position: &'a [i16],
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    pub model_index: i16,
}

// TE_BEAMENTS 8
#[derive(Clone, Debug)]
pub struct TeBeamEnts<'a> {
    // [i16; 3]
    pub start_entity: i16,
    pub end_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_SPARKS 9
#[derive(Clone, Debug)]
pub struct TeSparks<'a> {
    // [i16; 3]
    pub position: &'a [i16],
}

// TE_LAVASPLASH 10
#[derive(Clone, Debug)]
pub struct TeLavaSplash<'a> {
    // [i16; 3]
    pub position: &'a [i16],
}

// TE_TELEPORT 11
#[derive(Clone, Debug)]
pub struct TeTeleport<'a> {
    // [i16; 3]
    pub position: &'a [i16],
}

// TE_EXPLOSION2 12
#[derive(Clone, Debug)]
pub struct TeExplosion2<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub color: u8,
    pub count: u8,
}

// TE_BSPDECAL 13
#[derive(Clone, Debug)]
pub struct TeBspDecal<'a> {
    // [u8; 8]
    pub unknown1: &'a [u8],
    pub entity_index: i16,
    // [u8; 2]
    pub unknown2: Option<&'a [u8]>,
}

// TE_IMPLOSION 14
#[derive(Clone, Debug)]
pub struct TeImplosion<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub radius: u8,
    pub count: u8,
    pub life: u8,
}

// TE_SPRITETRAIL 15
#[derive(Clone, Debug)]
pub struct TeSpriteTrail<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub end_position: &'a [i16],
    pub sprite_index: i16,
    pub count: u8,
    pub life: u8,
    pub scale: u8,
    pub velocity: u8,
    pub velocity_randomness: u8,
}

// TE_SPRITE 16
#[derive(Clone, Debug)]
pub struct TeSprite<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub sprite_index: i16,
    pub scale: u8,
    pub brightness: u8,
}

// TE_BEAMSPRITE 18
#[derive(Clone, Debug)]
pub struct TeBeamSprite<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    pub end_position: &'a [i16],
    pub beam_sprite_index: i16,
    pub end_sprite_index: i16,
}

// TE_BEAMTORUS 19
#[derive(Clone, Debug)]
pub struct TeBeamTorus<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub axis: &'a [i16],
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_BEAMDISK 20
#[derive(Clone, Debug)]
pub struct TeBeamDisk<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub axis: &'a [i16],
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_BEAMCYLINDER 21
#[derive(Clone, Debug)]
pub struct TeBeamCylinder<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub axis: &'a [i16],
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_BEAMFOLLOW 22
#[derive(Clone, Debug)]
pub struct TeBeamFollow<'a> {
    pub start_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub life: u8,
    pub width: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
}

// TE_GLOWSPRITE 23
#[derive(Clone, Debug)]
pub struct TeGlowSprite<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub model_index: i16,
    pub scale: u8,
    pub size: u8,
    pub brightness: u8,
}

// TE_BEAMRING 24
#[derive(Clone, Debug)]
pub struct TeBeamRing<'a> {
    pub start_entity: i16,
    pub end_entity: i16,
    pub sprite_index: i16,
    pub start_frame: u8,
    pub frame_rate: u8,
    pub life: u8,
    pub width: u8,
    pub noise: u8,
    // [i16; 4] RGBA
    pub color: &'a [u8],
    pub speed: u8,
}

// TE_STREAKSPLASH 25
#[derive(Clone, Debug)]
pub struct TeStreakSplash<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub vector: &'a [i16],
    pub color: i16,
    pub count: u8,
    pub velocity: i16,
    pub velocity_randomness: i16,
}
// TE_DLIGHT 27
#[derive(Clone, Debug)]
pub struct TeDLight<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub radius: u8,
    // [i16; 3]
    pub color: &'a [u8],
    pub life: u8,
    pub decay_rate: u8,
}

// TE_ELIGHT 28
#[derive(Clone, Debug)]
pub struct TeELight<'a> {
    pub entity_index: i16,
    // [i16; 3]
    pub position: &'a [i16],
    pub radius: i16,
    // [i8; 3]
    pub color: &'a [u8],
    pub life: u8,
    pub decay_rate: i16,
}
// TE_TEXTMESSAGE 29
#[derive(Clone, Debug)]
pub struct TeTextMessage<'a> {
    pub channel: i8,
    pub x: i16,
    pub y: i16,
    pub effect: i8,
    // [u8; 4]
    pub text_color: &'a [u8],
    // THE docs forgot to mention this
    pub effect_color: &'a [u8],
    pub fade_in_time: i16,
    pub fade_out_time: i16,
    pub hold_time: i16,
    pub effect_time: Option<i16>,
    pub message: &'a [u8],
}

// TE_LINE 30
#[derive(Clone, Debug)]
pub struct TeLine<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub end_position: &'a [i16],
    pub life: i16,
    // [i8; 3]
    pub color: &'a [u8],
}

// TE_BOX 31
#[derive(Clone, Debug)]
pub struct TeBox<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub end_position: &'a [i16],
    pub life: i16,
    // [i8; 3]
    pub color: &'a [u8],
}

// TE_KILLBEAM 99
#[derive(Clone, Debug)]
pub struct TeKillBeam {
    pub entity_index: i16,
}

// TE_LARGEFUNNEL 100
#[derive(Clone, Debug)]
pub struct TeLargeFunnel<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    pub entity_index: i16,
    pub flags: i16,
}

// TE_BLOODSTREAM 101
#[derive(Clone, Debug)]
pub struct TeBloodStream<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub vector: i16,
    pub color: u8,
    pub count: u8,
}

// TE_SHOWLINE 102
#[derive(Clone, Debug)]
pub struct TeShowLine<'a> {
    // [i16; 3]
    pub start_position: &'a [i16],
    // [i16; 3]
    pub end_position: &'a [i16],
}

// TE_BLOOD 103
#[derive(Clone, Debug)]
pub struct TeBlood<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub vector: i16,
    pub color: u8,
    pub count: u8,
}

// TE_DECAL 104
#[derive(Clone, Debug)]
pub struct TeDecal<'a> {
    // [i16; 3]
    pub positiion: &'a [i16],
    pub decal_index: u8,
    pub entity_index: i16,
}

// TE_FIZZ 105
#[derive(Clone, Debug)]
pub struct TeFizz {
    pub entity_index: i16,
    pub model_index: i16,
    pub scale: u8,
}

// TE_MODEL 106
#[derive(Clone, Debug)]
pub struct TeModel<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub velocity: &'a [i16],
    pub angle_yaw: u8,
    pub model_index: i16,
    pub flags: u8,
    pub life: u8,
}

// TE_EXPLODEMODEL 107
#[derive(Clone, Debug)]
pub struct TeExplodeModel<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub velocity: &'a [i16],
    pub model_index: i16,
    pub count: i16,
    pub life: u8,
}

// TE_BREAKMODEL 108
#[derive(Clone, Debug)]
pub struct TeBreakModel<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub size: &'a [i16],
    // [i16; 3]
    pub velocity: &'a [i16],
    pub velocity_randomness: u8,
    pub object_index: i16,
    pub count: u8,
    pub life: u8,
    pub flags: u8,
}

// TE_GUNSHOTDECAL 109
#[derive(Clone, Debug)]
pub struct TeGunshotDecal<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub entity_index: i16,
    pub decal: u8,
}

// TE_SPRITESPRAY 110
#[derive(Clone, Debug)]
pub struct TeSpriteSpray<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub velocity: &'a [i16],
    pub model_index: i16,
    pub count: u8,
    pub speed: u8,
    pub random: u8,
}

// TE_ARMORRICOCHET 111
#[derive(Clone, Debug)]
pub struct TeArmorRicochet<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub scale: u8,
}

// TE_PLAYERDECAL 112
#[derive(Clone, Debug)]
pub struct TePlayerDecal<'a> {
    pub player_index: u8,
    // [i16; 3]
    pub position: &'a [i16],
    pub entity_index: i16,
    pub decal_index: u8,
}

// TE_BUBBLES 113
#[derive(Clone, Debug)]
pub struct TeBubbles<'a> {
    // [i16; 3]
    pub min_start_positition: &'a [i16],
    // [i16; 3]
    pub max_start_position: &'a [i16],
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub speed: i16,
}

// TE_BUBBLETRAIL 114
#[derive(Clone, Debug)]
pub struct TeBubbleTrail<'a> {
    // [i16; 3]
    pub min_start_positition: &'a [i16],
    // [i16; 3]
    pub max_start_position: &'a [i16],
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub speed: i16,
}

// TE_BLOODSPRITE 115
#[derive(Clone, Debug)]
pub struct TeBloodSprite<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub model_index: i16,
    pub decal_index: i16,
    pub color: u8,
    pub scale: u8,
}

// TE_WORLDDECAL 116
#[derive(Clone, Debug)]
pub struct TeWorldDecal<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub texture_index: u8,
}

// TE_WORLDDECALHIGH 117
#[derive(Clone, Debug)]
pub struct TeWorldDecalHigh<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub texture_index: u8,
}

// TE_DECALHIGH 118
#[derive(Clone, Debug)]
pub struct TeDecalHigh<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    pub decal_index: u8,
    pub entity_index: i16,
}

// TE_PROJECTILE 119
#[derive(Clone, Debug)]
pub struct TeProjectile<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub velocity: &'a [i16],
    pub model_index: i16,
    pub life: u8,
    pub color: u8,
}

// TE_SPRAY 120
#[derive(Clone, Debug)]
pub struct TeSpray<'a> {
    // [i16; 3]
    pub position: &'a [i16],
    // [i16; 3]
    pub direction: &'a [i16],
    pub model_index: i16,
    pub count: u8,
    pub life: u8,
    pub owner: u8,
}

// TE_PLAYERSPRITES 121
#[derive(Clone, Debug)]
pub struct TePlayerSprites {
    pub entity_index: i16,
    pub model_index: i16,
    pub count: u8,
    pub variance: u8,
}

// TE_PARTICLEBURST 122
#[derive(Clone, Debug)]
pub struct TeParticleBurst<'a> {
    // [i16; 3]
    pub origin: &'a [i16],
    pub scale: i16,
    pub color: u8,
    pub duration: u8,
}

// TE_FIREFIELD 123
#[derive(Clone, Debug)]
pub struct TeFireField<'a> {
    // [i16; 3]
    pub origin: &'a [i16],
    pub scale: i16,
    pub model_index: i16,
    pub count: u8,
    pub flags: u8,
    pub duration: u8,
}

// TE_PLAYERATTACHMENT 124
#[derive(Clone, Debug)]
pub struct TePlayerAttachment {
    pub entity_index: u8,
    pub scale: i16,
    pub model_index: i16,
    pub life: i16,
}

// TE_KILLPLAYERATTACHMENT 125
#[derive(Clone, Debug)]
pub struct TeKillPlayerAttachment {
    pub entity_index: u8,
}

// TE_MULTIGUNSHOT 126
#[derive(Clone, Debug)]
pub struct TeMultigunShot<'a> {
    // [i16; 3]
    pub origin: &'a [i16],
    // [i16; 3]
    pub direction: &'a [i16],
    // [i16; 2]
    pub noise: &'a [i16],
    pub count: u8,
    pub decal_index: u8,
}

// TE_USERTRACER 127
#[derive(Clone, Debug)]
pub struct TeUserTracer<'a> {
    // [i16; 3]
    pub origin: &'a [i16],
    // [i16; 3]
    pub velocity: &'a [i16],
    pub life: u8,
    pub color: u8,
    pub scale: u8,
}

/// SVC_SETPAUSE 24
#[derive(Clone, Debug)]
pub struct SvcSetPause {
    pub is_paused: i8,
}

/// SVC_SIGNONNUM 25
#[derive(Clone, Debug)]
pub struct SvcSignOnNum {
    pub sign: i8,
}

/// SVC_CENTERPRINT 26
#[derive(Clone, Debug)]
pub struct SvcCenterPrint<'a> {
    pub message: &'a [u8],
}

/// SVC_KILLEDMONSTER 27
// #[derive(Clone, Debug)]

/// SVC_FOUNDSECRET 28
// #[derive(Clone, Debug)]

/// SVC_SPAWNSTATICSOUND 29
#[derive(Clone, Debug)]
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
// #[derive(Clone, Debug)]

/// SVC_FINALE 31
#[derive(Clone, Debug)]
pub struct SvcFinale<'a> {
    pub text: &'a [u8],
}

/// SVC_CDTRACK 32
#[derive(Clone, Debug)]
pub struct SvcCdTrack {
    pub track: i8,
    pub loop_track: i8,
}

/// SVC_RESTORE 33
#[derive(Clone, Debug)]
pub struct SvcRestore<'a> {
    pub save_name: &'a [u8],
    pub map_count: u8,
    pub map_names: Vec<&'a [u8]>,
}

/// SVC_CUTSCENE 34
#[derive(Clone, Debug)]
pub struct SvcCutscene<'a> {
    pub text: &'a [u8],
}

/// SVC_WEAPONANIM 35
#[derive(Clone, Debug)]
pub struct SvcWeaponAnim {
    pub sequence_number: i8,
    pub weapon_model_body_group: i8,
}

/// SVC_DECALNAME 36
#[derive(Clone, Debug)]
pub struct SvcDecalName<'a> {
    pub position_index: u8,
    pub decal_name: &'a [u8],
}

/// SVC_ROOMTYPE 37
#[derive(Clone, Debug)]
pub struct SvcRoomType {
    pub room_type: u16,
}

/// SVC_ADDANGLE 38
#[derive(Clone, Debug)]
pub struct SvcAddAngle {
    pub angle_to_add: i16,
}

/// SVC_NEWUSERMSG 39
#[derive(Debug, Clone)]
pub struct SvcNewUserMsg<'a> {
    pub index: u8,
    // weird but it's for consistency
    pub size: i8,
    // [u8; 16]
    pub name: &'a [u8],
}

/// SVC_PACKETENTITIES 40
#[derive(Clone, Debug)]
pub struct SvcPacketEntities {
    // [bool; 16]
    pub entity_count: BitType,
    pub entity_states: Vec<EntityState>,
}

#[derive(Clone, Debug)]
pub struct EntityState {
    pub entity_index: u16,
    pub increment_entity_number: bool,
    pub is_absolute_entity_index: Option<bool>,
    // [bool; 11]
    pub absolute_entity_index: Option<BitType>,
    // [bool; 6]
    pub entity_index_difference: Option<BitType>,
    pub has_custom_delta: bool,
    pub has_baseline_index: bool,
    // [bool; 6]
    pub baseline_index: Option<BitType>,
    pub delta: Delta,
}

/// SVC_DELTAPACKETENTITIES 41
#[derive(Clone, Debug)]
pub struct SvcDeltaPacketEntities {
    // [bool; 16]
    pub entity_count: BitType,
    // [bool; 8]
    pub delta_sequence: BitType,
    pub entity_states: Vec<EntityStateDelta>,
}

/// These infos are not like THE docs mention.
#[derive(Clone, Debug)]
pub struct EntityStateDelta {
    // [bool; 11] but do u16 because arithmetic.
    pub entity_index: u16,
    pub remove_entity: bool,
    pub is_absolute_entity_index: bool,
    // [bool; 11]
    pub absolute_entity_index: Option<BitType>,
    // [bool; 6]
    pub entity_index_difference: Option<BitType>,
    // Need to be optional because if remove is true then it won't have delta.
    pub has_custom_delta: Option<bool>,
    pub delta: Option<Delta>,
}

/// SVC_CHOKE 42

/// SVC_RESOURCELIST 43
#[derive(Clone, Debug)]
pub struct SvcResourceList {
    // [bool; 12]
    pub resource_count: BitType,
    pub resources: Vec<Resource>,
    pub consistencies: Vec<Consistency>,
}

#[derive(Clone, Debug)]
pub struct Resource {
    // [bool; 4]
    pub type_: BitType,
    // &'[u8]
    pub name: BitType,
    // [bool; 12]
    pub index: BitType,
    // [bool; 24]
    pub size: BitType,
    // [bool; 3]
    pub flags: BitType,
    // [bool; 128]
    pub md5_hash: Option<BitType>,
    pub has_extra_info: bool,
    // [bool; 256]
    pub extra_info: Option<BitType>,
}

#[derive(Clone, Debug)]
pub struct Consistency {
    pub has_check_file_flag: bool,
    pub is_short_index: Option<bool>,
    // [bool; 5]
    pub short_index: Option<BitType>,
    // [bool; 10]
    pub long_index: Option<BitType>,
}

/// SVC_NEWMOVEVARS 44
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct SvcResourceRequest {
    pub spawn_count: i32,
    pub unknown: Vec<u8>,
}

/// SVC_CUSTOMIZATION 46
#[derive(Clone, Debug)]
pub struct SvcCustomization<'a> {
    pub player_index: u8,
    pub type_: u8,
    pub name: &'a [u8],
    pub index: u16,
    pub download_size: u32,
    pub flags: u8,
    // [u8; 16]
    pub md5_hash: Option<&'a [u8]>,
}

/// SVC_CROSSHAIRANGLE 47
#[derive(Clone, Debug)]
pub struct SvcCrosshairAngle {
    pub pitch: i16,
    pub yaw: i16,
}

/// SVC_SOUNDFADE 48
#[derive(Clone, Debug)]
pub struct SvcSoundFade {
    pub initial_percent: u8,
    pub hold_time: u8,
    pub fade_out_time: u8,
    pub fade_in_time: u8,
}

/// SVC_FILETXFERFAILED 49
#[derive(Clone, Debug)]
pub struct SvcFileTxferFailed<'a> {
    pub file_name: &'a [u8],
}

/// SVC_HLTV 50
#[derive(Clone, Debug)]
pub struct SvcHltv {
    pub mode: u8,
}

/// SVC_DIRECTOR 51
#[derive(Clone, Debug)]
pub struct SvcDirector<'a> {
    pub length: u8,
    pub flag: u8,
    pub message: &'a [u8],
}

/// SVC_VOINCEINIT 52
#[derive(Clone, Debug)]
pub struct SvcVoiceInit<'a> {
    pub codec_name: &'a [u8],
    pub quality: i8,
}

/// SVC_VOICEDATA 53
#[derive(Clone, Debug)]
pub struct SvcVoiceData<'a> {
    pub player_index: u8,
    pub size: u16,
    pub data: &'a [u8],
}

/// SVC_SENDEXTRAINFO 54
#[derive(Clone, Debug)]
pub struct SvcSendExtraInfo<'a> {
    pub fallback_dir: &'a [u8],
    pub can_cheat: u8,
}

/// SVC_TIMESCALE 55
#[derive(Clone, Debug)]
pub struct SvcTimeScale {
    pub time_scale: f32,
}

/// SVC_RESOURCELOCATION 56
#[derive(Clone, Debug)]
pub struct SvcResourceLocation<'a> {
    pub download_url: &'a [u8],
}

/// SVC_SENDCVARVALUE 57
#[derive(Clone, Debug)]
pub struct SvcSendCvarValue<'a> {
    pub name: &'a [u8],
}

/// SVC_SENDCVARVALUE2 58
#[derive(Clone, Debug)]
pub struct SvcSendCvarValue2<'a> {
    pub request_id: u32,
    pub name: &'a [u8],
}

#[derive(Clone, Debug)]
pub enum Message<'a> {
    UserMessage(NetMsgUserMessage<'a>),
    EngineMessage(EngineMessage<'a>),
}

pub enum MessageType {
    UserMessage,
    EngineMessageType(EngineMessageType),
}

#[repr(u8)]
#[derive(Clone, Debug)]
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
    SvcUpdateUserInfo(SvcUpdateUserInfo<'a>) = 13,
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
    SvcSpawnStaticSound(SvcSpawnStaticSound) = 29,
    SvcIntermission = 30,
    SvcFinale(SvcFinale<'a>) = 31,
    SvcCdTrack(SvcCdTrack) = 32,
    SvcRestore(SvcRestore<'a>) = 33,
    SvcCutscene(SvcCutscene<'a>) = 34,
    SvcWeaponAnim(SvcWeaponAnim) = 35,
    SvcDecalName(SvcDecalName<'a>) = 36,
    SvcRoomType(SvcRoomType) = 37,
    SvcAddAngle(SvcAddAngle) = 38,
    SvcNewUserMsg(SvcNewUserMsg<'a>) = 39,
    SvcPacketEntities(SvcPacketEntities) = 40,
    SvcDeltaPacketEntities(SvcDeltaPacketEntities) = 41,
    SvcChoke = 42,
    SvcResourceList(SvcResourceList) = 43,
    SvcNewMovevars(SvcNewMoveVars<'a>) = 44,
    SvcResourceRequest(SvcResourceRequest) = 45,
    SvcCustomization(SvcCustomization<'a>) = 46,
    SvcCrosshairAngle(SvcCrosshairAngle) = 47,
    SvcSoundFade(SvcSoundFade) = 48,
    SvcFileTxferFailed(SvcFileTxferFailed<'a>) = 49,
    SvcHltv(SvcHltv) = 50,
    SvcDirector(SvcDirector<'a>) = 51,
    SvcVoiceInit(SvcVoiceInit<'a>) = 52,
    SvcVoiceData(SvcVoiceData<'a>) = 53,
    SvcSendExtraInfo(SvcSendExtraInfo<'a>) = 54,
    SvcTimeScale(SvcTimeScale) = 55,
    SvcResourceLocation(SvcResourceLocation<'a>) = 56,
    SvcSendCvarValue(SvcSendCvarValue<'a>) = 57,
    SvcSendCvarValue2(SvcSendCvarValue2<'a>) = 58,
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
            59 => MessageType::EngineMessageType(EngineMessageType::SvcBad),
            60 => MessageType::EngineMessageType(EngineMessageType::SvcBad),
            61 => MessageType::EngineMessageType(EngineMessageType::SvcBad),
            62 => MessageType::EngineMessageType(EngineMessageType::SvcBad),
            63 => MessageType::EngineMessageType(EngineMessageType::SvcBad),
            _ => MessageType::UserMessage,
        }
    }
}
