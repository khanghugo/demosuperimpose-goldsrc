// delta.lst

// SVC_SERVERINFO 11
#[derive(Debug)]
pub struct ServerInfo<'a> {
    pub protocol: i32,
    pub spawn_count: i32,
    pub map_checksum: i32,
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

// SVC_PACKETENTITIES (40)
struct PacketEntities<'a> {
    entity_count: u16, // supposed to be 16 bits
    entity_states: &'a [EntityState],
}

struct EntityState {
    increment_entity_number: bool,
    is_absolute_entity_index: Option<bool>,
    absolute_entity_index: Option<[bool; 18]>,
    entity_index_difference: Option<[bool; 6]>,
    has_custom_delta: bool,
    has_baseline_index: bool,
    // delta: Option<
}

#[repr(u8)]
pub enum EngineMessageType<'a> {
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
    SvcServerInfo(ServerInfo<'a>) = 11,
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
    SvcResourceAllocation = 56,
    SvcSendCvarValue = 57,
    SvcSendCvarValue2 = 58,
}
