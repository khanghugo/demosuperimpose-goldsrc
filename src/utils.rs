use hldemo::{MoveVars, NetMsgData, NetMsgInfo, RefParams, UserCmd};

#[macro_export]
macro_rules! write_demo {
    ($demo_name:literal, $demo:ident) => {{
        use demosuperimpose_goldsrc::writer::DemoWriter;
        let mut out = DemoWriter::new(String::from($demo_name));
        out.write_file($demo);
    }};
}

#[macro_export]
macro_rules! wrap_message {
    ($svc:ident, $msg:ident) => {{
        let huh = EngineMessage::$svc($msg);
        let hah = Message::EngineMessage(huh);
        hah
    }};
}

#[macro_export]
macro_rules! open_demo {
    ($name:literal) => {{
        let mut bytes = Vec::new();
        let mut f = File::open($name).unwrap();
        f.read_to_end(&mut bytes).unwrap();

        hldemo::Demo::parse(bytes.leak()).unwrap()
    }};

    ($name:ident) => {{
        let mut bytes = Vec::new();
        let mut f = File::open($name).unwrap();
        f.read_to_end(&mut bytes).unwrap();

        hldemo::Demo::parse(bytes.leak()).unwrap()
    }};
}

#[macro_export]
macro_rules! nbit_num {
    ($num:expr, $bit:expr) => {{
        use crate::writer::BitWriter;
        let mut writer = BitWriter::new();
        writer.append_u32_range($num as u32, $bit);
        writer.data
    }};
}

#[macro_export]
macro_rules! nbit_str {
    ($name:expr) => {{
        use crate::writer::BitWriter;
        let mut writer = BitWriter::new();
        $name.as_bytes().iter().for_each(|s| writer.append_u8(*s));
        writer.data
    }};
}

#[macro_export]
macro_rules! init_parse {
    ($demo:ident) => {{
        use crate::demo_doer::{
            get_initial_delta, parse_netmsg, FrameData, HashMap, SvcNewUserMsg,
        };

        let mut delta_decoders = get_initial_delta();
        let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

        // use hldemo::Demo;
        for frame in $demo
            .directory
            .entries
            .get_mut(0)
            .unwrap()
            .frames
            .iter_mut()
        {
            match &mut frame.data {
                FrameData::NetMsg((_, data)) => {
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();
                }
                _ => (),
            }
        }

        (delta_decoders, custom_messages)
    }};
}

// usually spawnbaseline is very packed so there won't be gaps unlike packet entities
#[macro_export]
macro_rules! append_spawnbaseline {
    ($entity_arr:ident,$entity:ident) => {{
        // Find free entities indices.
        let mut current_free_entity = 0;
        let mut insert_idx = 0;

        for (idx, ent) in $entity_arr.iter().enumerate() {
            if ent.index.to_u16() == current_free_entity {
                current_free_entity += 1;
                insert_idx = idx + 1;
            } else {
                break;
            }
        }

        // current_free_entity is a free entity

        // Insert new baseline.
        let other_demo_entity_idx = nbit_num!(current_free_entity, 11);
        let other_demo_type = nbit_num!(1, 2);

        entity_arr.insert(
            insert_idx,
            EntityS {
                entity_index: other_demo_entity_idx.to_u16(),
                index: other_demo_entity_idx,
                type_: other_demo_type,
                delta: $entity,
            },
        );
    }};
}

#[macro_export]
macro_rules! append_packet_entities {
    ($packet:ident,$entity:ident) => {{
        $packet.entity_count = nbit_num!(packet.entity_count.to_u32() + 1, 16);

        let mut other_demo_entity_state_delta = Delta::new();

        // Find free entity.
        let mut insert_index = 0;
        for entity in &packet.entity_states {
            if entity.entity_index > ghost.get_entity_index() {
                break;
            }

            if entity.entity_index > insert_index {
                break;
            }

            insert_index += 1;
        }

        // Insert between inserted entity and the one before it
        // due to entity index difference mechanism.
        let before_entity = &packet.entity_states[insert_index - 1];
        let mut is_absolute_entity_index = false;
        let mut ghost_absolute_entity_index: Option<BitType> = None;
        let mut ghost_entity_index_difference: Option<BitType> = None;

        // If difference is more than 63, we do absolute entity index instead.
        // The reason is that difference is only 6 bits, so 63 max.
        let difference = insert_index - before_entity.entity_index;
        if difference > (1 << 6) - 1 {
            ghost_absolute_entity_index = Some(nbit_num!(insert_index, 11));
            is_absolute_entity_index = true;
        } else {
            ghost_entity_index_difference =
                Some(nbit_num!(insert_index - before_entity.entity_index, 6));
        }

        let other_demo_entity_state = EntityStateDelta {
            entity_index: insert_index, // This doesn't really do anything but for you to read.
            remove_entity: false,
            is_absolute_entity_index,
            absolute_entity_index: ghost_absolute_entity_index,
            entity_index_difference: ghost_entity_index_difference,
            has_custom_delta: Some(false),
            delta: Some(entity),
        };

        // Insert between inserted entity and next entity.
        // If it is last entity then there is no need to change.
        if insert_index < packet.entity_states.len() {
            let next_entity = &mut packet.entity_states[insert_index];
            let difference = next_entity.entity_index - insert_index;

            if difference > (1 << 6) - 1 {
                // It is possible that by the time this is hit,
                // the next entity is already numbered by absolute index.
            } else {
                next_entity.entity_index_difference = Some(nbit_num!(difference, 6));
            }
        }

        packet
            .entity_states
            .insert(insert_index, other_demo_entity_state);
    }};
}

#[macro_export]
macro_rules! rand_int_range {
    ($x1:expr,$x2:expr) => {{
        use rand::prelude::*;
        (rand::random::<f32>() * ($x2 - $x1) as f32 + $x1 as f32).round() as u32
    }};
}

#[repr(u16)]
pub enum Buttons {
    Attack = 1 << 0,
    Jump = 1 << 1,
    Duck = 1 << 2,
    Forward = 1 << 3,
    Back = 1 << 4,
    Use = 1 << 5,
    Cancel = 1 << 6,
    Left = 1 << 7,
    Right = 1 << 8,
    MoveLeft = 1 << 9,
    MoveRight = 1 << 10,
    Attack2 = 1 << 11,
    Run = 1 << 12,
    Reload = 1 << 13,
    Alt1 = 1 << 14,
    Score = 1 << 15,
}

pub fn decode_buttons(buttons: u16) -> Vec<Buttons> {
    let mut res: Vec<Buttons> = vec![];

    for i in 0..16 {
        let curr = 1u16 << i;
        if curr & buttons != 0 {
            res.push(match i {
                0 => Buttons::Attack,
                1 => Buttons::Jump,
                2 => Buttons::Duck,
                3 => Buttons::Forward,
                4 => Buttons::Back,
                5 => Buttons::Use,
                6 => Buttons::Cancel,
                7 => Buttons::Left,
                8 => Buttons::Right,
                9 => Buttons::MoveLeft,
                10 => Buttons::MoveRight,
                11 => Buttons::Attack2,
                12 => Buttons::Run,
                13 => Buttons::Reload,
                14 => Buttons::Alt1,
                15 => Buttons::Score,
                _ => unreachable!(),
            })
        }
    }

    res
}

pub fn encode_buttons(buttons: Vec<Buttons>) -> u16 {
    let mut res: u16 = 0;

    for what in buttons {
        res |= what as u16;
    }

    res
}

#[repr(u32)]
pub enum EdictFlags {
    Fly = (1 << 0),
    Swim = (1 << 1),
    Conveyor = (1 << 2),
    Client = (1 << 3),
    InWater = (1 << 4),
    Monster = (1 << 5),
    GodMode = (1 << 6),
    Notarget = (1 << 7),
    SkipLocalHost = (1 << 8),
    Onground = (1 << 9),
    PartialGround = (1 << 10),
    WaterJump = (1 << 11),
    Frozen = (1 << 12),
    FakeClient = (1 << 13),
    Ducking = (1 << 14),
    Float = (1 << 15),
    Graphed = (1 << 16),
    ImmuneWater = (1 << 17),
    ImmuneSlime = (1 << 18),
    ImmuneLava = (1 << 19),
    Proxy = (1 << 20),
    AlwaysThink = (1 << 21),
    BaseVelocity = (1 << 22),
    MonsterClip = (1 << 23),
    OnTrain = (1 << 24),
    WorldBrush = (1 << 25),
    Spectator = (1 << 26),
    // There are no 27 28
    CustomEntity = (1 << 29),
    KillMe = (1 << 30),
    Dormant = (1 << 31),
}

pub fn decode_edict_flags(flags: u32) -> Vec<EdictFlags> {
    let mut res: Vec<EdictFlags> = vec![];

    for i in 0..32 {
        let curr = 1u32 << i;
        if curr & flags != 0 {
            res.push(match i {
                0 => EdictFlags::Fly,
                1 => EdictFlags::Swim,
                2 => EdictFlags::Conveyor,
                3 => EdictFlags::Client,
                4 => EdictFlags::InWater,
                5 => EdictFlags::Monster,
                6 => EdictFlags::GodMode,
                7 => EdictFlags::Notarget,
                8 => EdictFlags::SkipLocalHost,
                9 => EdictFlags::Onground,
                10 => EdictFlags::PartialGround,
                11 => EdictFlags::WaterJump,
                12 => EdictFlags::Frozen,
                13 => EdictFlags::FakeClient,
                14 => EdictFlags::Ducking,
                15 => EdictFlags::Float,
                16 => EdictFlags::Graphed,
                17 => EdictFlags::ImmuneWater,
                18 => EdictFlags::ImmuneSlime,
                19 => EdictFlags::ImmuneLava,
                20 => EdictFlags::Proxy,
                21 => EdictFlags::AlwaysThink,
                22 => EdictFlags::BaseVelocity,
                23 => EdictFlags::MonsterClip,
                24 => EdictFlags::OnTrain,
                25 => EdictFlags::WorldBrush,
                26 => EdictFlags::Spectator,
                29 => EdictFlags::CustomEntity,
                30 => EdictFlags::KillMe,
                31 => EdictFlags::Dormant,
                _ => unreachable!(),
            })
        }
    }

    res
}

pub fn encode_edict_flags(flags: Vec<EdictFlags>) -> u32 {
    let mut res = 0u32;

    for what in flags {
        res |= what as u32;
    }

    res
}

// In delta.lst, it is an integer described with 4 bits so it is more than 4 bits.
#[repr(i32)]
pub enum MoveType {
    None = 0,
    AngleClip = 1,   // unused
    AngleNoClip = 2, // unused
    Walk = 3,
    Step = 4,
    Fly = 5,
    Toss = 6,
    Push = 7,
    Noclip = 8,
    Flymissile = 9,
    Bounce = 10,
    Bouncemissile = 11,
    Follow = 12,
    Pushstep = 13,
}

pub fn decode_movetype(movetype: i32) -> MoveType {
    match movetype {
        0 => MoveType::None,
        1 => MoveType::AngleClip,
        2 => MoveType::AngleNoClip,
        3 => MoveType::Walk,
        4 => MoveType::Step,
        5 => MoveType::Fly,
        6 => MoveType::Toss,
        7 => MoveType::Push,
        8 => MoveType::Noclip,
        9 => MoveType::Flymissile,
        10 => MoveType::Bounce,
        11 => MoveType::Bouncemissile,
        12 => MoveType::Follow,
        13 => MoveType::Pushstep,
        _ => unreachable!(),
    }
}

#[repr(i32)]
pub enum CSWeapon {
    None = 0,
    P228 = 1,
    Glock = 2,
    Scout = 3,
    HeGrenade = 4,
    Xm1014 = 5,
    C4 = 6,
    Mac10 = 7,
    Aug = 8,
    SmokeGrenade = 9,
    Elite = 10,
    FiveSeven = 11,
    Ump45 = 12,
    Sg550 = 13,
    Galil = 14,
    Famas = 15,
    Usp = 16,
    Glock18 = 17,
    Awp = 18,
    Mp5n = 19,
    M249 = 20,
    M3 = 21,
    M4a1 = 22,
    Tmp = 23,
    G3sg1 = 24,
    Flashbang = 25,
    Deagle = 26,
    Sg552 = 27,
    Ak47 = 28,
    Knife = 29,
    P90 = 30,
    ShieldGun = 99,
}

pub fn decode_cs_weapon(weapon: i32) -> CSWeapon {
    match weapon {
        0 => CSWeapon::None,
        1 => CSWeapon::P228,
        2 => CSWeapon::Glock,
        3 => CSWeapon::Scout,
        4 => CSWeapon::HeGrenade,
        5 => CSWeapon::Xm1014,
        6 => CSWeapon::C4,
        7 => CSWeapon::Mac10,
        8 => CSWeapon::Aug,
        9 => CSWeapon::SmokeGrenade,
        10 => CSWeapon::Elite,
        11 => CSWeapon::FiveSeven,
        12 => CSWeapon::Ump45,
        13 => CSWeapon::Sg550,
        14 => CSWeapon::Galil,
        15 => CSWeapon::Famas,
        16 => CSWeapon::Usp,
        17 => CSWeapon::Glock18,
        18 => CSWeapon::Awp,
        19 => CSWeapon::Mp5n,
        20 => CSWeapon::M249,
        21 => CSWeapon::M3,
        22 => CSWeapon::M4a1,
        23 => CSWeapon::Tmp,
        24 => CSWeapon::G3sg1,
        25 => CSWeapon::Flashbang,
        26 => CSWeapon::Deagle,
        27 => CSWeapon::Sg552,
        28 => CSWeapon::Ak47,
        29 => CSWeapon::Knife,
        30 => CSWeapon::P90,
        99 => CSWeapon::ShieldGun,
        _ => unreachable!(),
    }
}

pub enum ResourceType {
    Sound = 0,
    Skin = 1,
    Model = 2,
    Decal = 3,
    Generic = 4,
    Eventscript = 5,
    World = 6,
}

pub trait NetMsgDataMethods {
    /// Creates semi-default net message data for CS 1.6
    ///
    /// Recommended to change fields after this. Or just add new method :DDD
    ///
    /// seq: Sequence number for the net message. It is to ensure that the demo won't crash.
    /// Try to use a value that is not `0` for it.
    fn new(seq: i32) -> Self;
}

const VEC_0: [f32; 3] = [0., 0., 0.];
const VIEWHEIGHT: [f32; 3] = [0.0, 0.0, 17.0];
const VIEWPORT: [i32; 4] = [0, 0, 1024, 768];
const SKYNAME: [u8; 32] = [0u8; 32];
// const seq: i32 = 69;

impl<'a> NetMsgDataMethods for NetMsgData<'a> {
    fn new(seq: i32) -> Self {
        Self {
            info: NetMsgInfo {
                timestamp: 0.0,
                ref_params: RefParams {
                    vieworg: VEC_0,
                    viewangles: VEC_0,
                    forward: VEC_0,
                    right: VEC_0,
                    up: VEC_0,
                    frametime: 0.,
                    time: 0.,
                    intermission: 0,
                    paused: 0,
                    spectator: 0,
                    onground: 0,
                    waterlevel: 0,
                    simvel: VEC_0,
                    simorg: VEC_0,
                    viewheight: VIEWHEIGHT,
                    idealpitch: 0.,
                    cl_viewangles: VEC_0,
                    health: 100,
                    crosshairangle: VEC_0,
                    viewsize: 120.,
                    punchangle: VEC_0,
                    maxclients: 32,
                    viewentity: 1,
                    playernum: 0,
                    max_entities: 6969,
                    demoplayback: 0,
                    hardware: 1,
                    smoothing: 1,
                    ptr_cmd: 0,
                    ptr_movevars: 0,
                    viewport: VIEWPORT,
                    next_view: 0,
                    only_client_draw: 0,
                },
                usercmd: UserCmd {
                    lerp_msec: 9,
                    msec: 10,
                    viewangles: VEC_0,
                    forwardmove: 0.,
                    sidemove: 0.,
                    upmove: 0.,
                    lightlevel: 68,
                    buttons: 0,
                    impulse: 0,
                    weaponselect: 0,
                    impact_index: 0,
                    impact_position: VEC_0,
                },
                movevars: MoveVars {
                    gravity: 800.0,
                    stopspeed: 75.0,
                    maxspeed: 320.,
                    spectatormaxspeed: 500.,
                    accelerate: 5.,
                    airaccelerate: 10.,
                    wateraccelerate: 10.,
                    friction: 4.,
                    edgefriction: 2.,
                    waterfriction: 1.,
                    entgravity: 1.,
                    bounce: 1.,
                    stepsize: 18.,
                    maxvelocity: 2000.,
                    zmax: 409600.,
                    wave_height: 0.,
                    footsteps: 1,
                    sky_name: &SKYNAME, // TODO
                    rollangle: 0.,
                    rollspeed: 0.,
                    skycolor_r: 0.,
                    skycolor_g: 0.,
                    skycolor_b: 0.,
                    skyvec_x: 0.,
                    skyvec_y: 0.,
                    skyvec_z: 0.,
                },
                view: VEC_0,
                viewmodel: 0,
            },
            // To make sure that game doesn't crash, change it like this.
            incoming_sequence: seq,
            incoming_acknowledged: seq - 1,
            incoming_reliable_acknowledged: 1,
            incoming_reliable_sequence: 0,
            outgoing_sequence: seq,
            reliable_sequence: 1,
            last_reliable_sequence: seq - 1,
            msg: &[],
        }
    }
}

#[macro_export]
macro_rules! get_cs_delta_msg {
    () => {{
        let d1 = SvcDeltaDescription {
            name: &[101, 118, 101, 110, 116, 95, 116, 0],
            total_fields: 14,
            fields: vec![],
            clone: &[
                249, 67, 0, 0, 0, 40, 115, 163, 75, 115, 35, 43, 195, 3, 32, 0, 8, 88, 0, 125, 0,
                0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 128, 24, 92, 152, 92, 88, 91, 12, 0, 16, 64, 64,
                0, 232, 3, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 196, 224, 194, 228, 194, 218,
                100, 0, 136, 0, 2, 2, 64, 31, 0, 0, 64, 31, 0, 0, 242, 71, 0, 0, 0, 248, 38, 151,
                118, 150, 230, 182, 5, 211, 5, 128, 0, 16, 160, 1, 0, 64, 31, 0, 250, 0, 0, 144,
                63, 2, 0, 0, 192, 55, 185, 180, 179, 52, 183, 173, 152, 46, 0, 6, 128, 0, 13, 0, 0,
                250, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 190, 201, 165, 157, 165, 185, 109, 201,
                116, 1, 64, 0, 4, 104, 0, 0, 208, 7, 128, 62, 0, 0, 228, 143, 0, 0, 0, 208, 12, 46,
                76, 46, 172, 45, 6, 0, 6, 32, 128, 2, 80, 195, 0, 0, 244, 1, 0, 32, 127, 4, 0, 0,
                128, 102, 112, 97, 114, 97, 109, 50, 0, 52, 0, 1, 20, 128, 26, 6, 0, 160, 15, 0, 0,
                249, 67, 0, 0, 0, 76, 131, 11, 147, 11, 107, 139, 1, 192, 1, 8, 144, 0, 125, 0, 0,
                0, 125, 0, 0, 200, 31, 2, 0, 0, 96, 26, 92, 152, 92, 88, 155, 12, 0, 15, 64, 128,
                4, 232, 3, 0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 195, 220, 206, 216, 202, 230, 182,
                96, 186, 0, 40, 0, 2, 52, 0, 0, 232, 3, 64, 31, 0, 0, 242, 71, 0, 0, 0, 24, 230,
                118, 198, 86, 54, 183, 21, 211, 5, 128, 1, 16, 160, 1, 0, 64, 31, 0, 250, 0, 0,
                144, 63, 2, 0, 0, 192, 48, 183, 51, 182, 178, 185, 45, 153, 46, 0, 14, 128, 0, 13,
                0, 0, 250, 0, 208, 7, 0, 128, 252, 33, 0, 0, 0, 144, 213, 141, 173, 165, 185, 157,
                1, 176, 0, 4, 4, 128, 62, 0, 0, 128, 62, 0, 0, 0,
            ],
        };

        let d2 = SvcDeltaDescription {
            name: &[
                119, 101, 97, 112, 111, 110, 95, 100, 97, 116, 97, 95, 116, 0,
            ],
            total_fields: 18,
            fields: vec![],
            clone: &[
                249, 35, 0, 0, 0, 108, 251, 50, 99, 163, 74, 107, 43, 187, 42, 11, 131, 123, 115,
                75, 34, 99, 43, 3, 128, 0, 8, 176, 0, 72, 232, 1, 0, 125, 0, 0, 200, 31, 1, 0, 0,
                96, 219, 151, 25, 155, 83, 25, 30, 29, 148, 92, 90, 91, 152, 92, 94, 16, 29, 93,
                216, 216, 26, 0, 2, 64, 128, 5, 64, 66, 15, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 219,
                190, 204, 216, 156, 202, 240, 232, 164, 202, 216, 222, 194, 200, 0, 56, 0, 2, 44,
                0, 18, 122, 0, 64, 31, 0, 0, 242, 71, 0, 0, 0, 216, 246, 101, 230, 84, 134, 71, 23,
                148, 214, 38, 244, 230, 86, 55, 7, 192, 2, 16, 96, 1, 144, 208, 3, 0, 250, 0, 0,
                144, 63, 2, 0, 0, 192, 182, 47, 51, 54, 167, 50, 60, 186, 169, 178, 177, 55, 55,
                178, 48, 185, 188, 32, 58, 186, 176, 177, 53, 0, 6, 128, 0, 11, 128, 132, 30, 0,
                208, 7, 0, 128, 252, 33, 0, 0, 0, 182, 125, 165, 13, 177, 165, 193, 1, 16, 0, 4,
                40, 128, 62, 0, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 176, 237, 203, 140, 13, 170,
                174, 13, 142, 42, 173, 173, 12, 0, 4, 32, 192, 2, 32, 161, 7, 0, 244, 1, 0, 32,
                127, 8, 0, 0, 0, 109, 95, 102, 73, 110, 83, 112, 101, 99, 105, 97, 108, 82, 101,
                108, 111, 97, 100, 0, 24, 0, 1, 2, 160, 15, 0, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0,
                104, 251, 50, 147, 42, 99, 123, 11, 35, 163, 74, 107, 43, 3, 32, 1, 8, 128, 0, 212,
                48, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 64, 219, 151, 89, 146, 155, 84, 25, 219, 91,
                24, 25, 0, 5, 64, 64, 0, 232, 3, 0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 218, 190,
                204, 130, 210, 218, 202, 200, 136, 194, 218, 194, 206, 202, 0, 80, 0, 2, 44, 0, 18,
                122, 0, 64, 31, 0, 0, 242, 135, 0, 0, 0, 208, 246, 101, 150, 228, 166, 245, 246,
                214, 6, 0, 3, 16, 128, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63, 4, 0, 0, 128, 182, 175,
                180, 171, 178, 48, 184, 55, 183, 41, 186, 48, 186, 50, 0, 26, 128, 128, 3, 208, 7,
                0, 0, 208, 7, 0, 128, 236, 33, 0, 0, 0, 180, 125, 165, 37, 145, 1, 4, 20, 128, 62,
                0, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 208, 172, 110, 174, 76, 46, 6, 0, 9, 32,
                192, 2, 32, 161, 7, 0, 244, 1, 0, 32, 127, 4, 0, 0, 128, 102, 117, 115, 101, 114,
                50, 0, 76, 0, 1, 22, 0, 208, 7, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0, 52, 171, 155,
                43, 147, 155, 1, 128, 2, 8, 176, 0, 128, 62, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 96,
                90, 221, 92, 153, 92, 12, 0, 14, 64, 0, 4, 0, 244, 1, 0, 232, 3, 0, 0,
            ],
        };

        let d3 = SvcDeltaDescription {
            name: &[117, 115, 101, 114, 99, 109, 100, 95, 116, 0],
            total_fields: 15,
            fields: vec![],
            clone: &[
                217, 19, 0, 0, 0, 96, 43, 147, 131, 251, 106, 155, 43, 27, 3, 8, 72, 0, 125, 0, 0,
                0, 125, 0, 0, 200, 95, 0, 0, 0, 64, 219, 92, 217, 24, 128, 0, 64, 0, 2, 232, 3, 0,
                0, 232, 3, 0, 64, 254, 32, 0, 0, 0, 236, 210, 202, 238, 194, 220, 206, 216, 202,
                230, 182, 98, 186, 0, 16, 0, 2, 32, 64, 31, 0, 0, 64, 31, 0, 0, 242, 7, 1, 0, 0,
                96, 151, 86, 118, 23, 230, 118, 198, 86, 54, 183, 5, 211, 5, 64, 0, 16, 0, 1, 250,
                0, 0, 0, 250, 0, 0, 144, 63, 1, 0, 0, 0, 177, 58, 58, 186, 55, 183, 57, 0, 15, 128,
                0, 8, 208, 7, 0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 154, 189, 201, 221, 133, 201,
                145, 181, 189, 217, 149, 1, 64, 0, 4, 48, 128, 62, 0, 0, 128, 62, 0, 0, 228, 47, 0,
                0, 0, 128, 45, 237, 12, 141, 142, 173, 204, 174, 140, 13, 128, 3, 32, 0, 1, 244, 1,
                0, 0, 244, 1, 0, 32, 127, 4, 0, 0, 128, 115, 105, 100, 101, 109, 111, 118, 101, 0,
                20, 0, 1, 12, 160, 15, 0, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0, 172, 131, 107, 123,
                179, 43, 3, 192, 0, 8, 96, 0, 125, 0, 0, 0, 125, 0, 0, 200, 95, 0, 0, 0, 64, 90,
                27, 92, 29, 219, 92, 25, 0, 8, 64, 0, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 32, 0,
                0, 0, 236, 210, 202, 238, 194, 220, 206, 216, 202, 230, 182, 100, 186, 0, 24, 0, 2,
                32, 64, 31, 0, 0, 64, 31, 0, 0, 242, 135, 0, 0, 0, 144, 214, 6, 23, 54, 70, 247,
                149, 230, 70, 86, 134, 7, 64, 2, 16, 96, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63, 2, 0,
                0, 192, 180, 54, 184, 176, 49, 186, 47, 184, 183, 185, 52, 186, 180, 55, 183, 45,
                152, 46, 0, 20, 128, 0, 8, 128, 62, 0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 166,
                181, 193, 133, 141, 209, 125, 193, 189, 205, 165, 209, 165, 189, 185, 109, 197,
                116, 1, 176, 0, 4, 64, 0, 244, 1, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 48, 173, 13,
                46, 108, 140, 238, 11, 238, 109, 46, 141, 46, 237, 205, 109, 75, 166, 11, 0, 6, 32,
                0, 2, 160, 15, 0, 0, 244, 1, 0, 0,
            ],
        };

        let d4 = SvcDeltaDescription {
            name: &[
                99, 117, 115, 116, 111, 109, 95, 101, 110, 116, 105, 116, 121, 95, 115, 116, 97,
                116, 101, 95, 116, 0,
            ],
            total_fields: 19,
            fields: vec![],
            clone: &[
                249, 67, 0, 0, 0, 144, 43, 115, 35, 43, 147, 107, 123, 35, 43, 3, 64, 2, 8, 64, 0,
                125, 0, 0, 0, 125, 0, 0, 200, 31, 1, 0, 0, 224, 155, 92, 218, 89, 154, 219, 22, 76,
                23, 0, 4, 64, 64, 4, 64, 31, 0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 223, 228, 210,
                206, 210, 220, 182, 98, 186, 0, 40, 0, 2, 34, 0, 250, 0, 0, 64, 31, 0, 0, 242, 71,
                0, 0, 0, 248, 38, 151, 118, 150, 230, 182, 37, 211, 5, 128, 1, 16, 16, 1, 208, 7,
                0, 0, 250, 0, 0, 144, 63, 2, 0, 0, 192, 48, 183, 51, 182, 178, 185, 45, 152, 46, 0,
                14, 128, 128, 8, 128, 62, 0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 134, 185, 157,
                177, 149, 205, 109, 197, 116, 1, 128, 0, 4, 68, 0, 244, 1, 0, 128, 62, 0, 0, 228,
                143, 0, 0, 0, 48, 204, 237, 140, 173, 108, 110, 75, 166, 11, 128, 4, 32, 32, 2,
                160, 15, 0, 0, 244, 1, 0, 32, 127, 8, 0, 0, 0, 115, 101, 113, 117, 101, 110, 99,
                101, 0, 44, 0, 1, 16, 160, 15, 0, 0, 160, 15, 0, 0, 249, 67, 0, 0, 0, 152, 91, 75,
                115, 3, 192, 1, 8, 128, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 64, 219, 27,
                89, 25, 91, 154, 27, 89, 25, 30, 0, 10, 64, 0, 4, 232, 3, 0, 0, 232, 3, 0, 64, 254,
                8, 0, 0, 0, 230, 198, 194, 216, 202, 0, 128, 0, 2, 16, 64, 31, 0, 0, 32, 3, 0, 0,
                242, 135, 0, 0, 0, 32, 246, 70, 150, 7, 64, 6, 16, 128, 0, 250, 0, 0, 0, 250, 0, 0,
                144, 191, 0, 0, 0, 0, 185, 50, 55, 178, 50, 185, 177, 55, 182, 55, 57, 23, 57, 0,
                40, 128, 0, 4, 208, 7, 0, 0, 208, 7, 0, 128, 252, 5, 0, 0, 0, 200, 149, 185, 145,
                149, 201, 141, 189, 177, 189, 201, 185, 156, 1, 68, 1, 4, 32, 128, 62, 0, 0, 128,
                62, 0, 0, 228, 47, 0, 0, 0, 64, 174, 204, 141, 172, 76, 110, 236, 141, 237, 77,
                206, 69, 12, 64, 10, 32, 0, 1, 244, 1, 0, 0, 244, 1, 0, 32, 127, 8, 0, 0, 0, 114,
                101, 110, 100, 101, 114, 102, 120, 0, 84, 0, 1, 8, 160, 15, 0, 0, 160, 15, 0, 0,
                249, 67, 0, 0, 0, 144, 43, 115, 35, 43, 147, 11, 107, 163, 3, 96, 2, 8, 64, 0, 125,
                0, 0, 0, 125, 0, 0, 200, 31, 1, 0, 0, 128, 153, 92, 88, 91, 25, 0, 12, 64, 0, 2,
                232, 3, 0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 194, 220, 210, 218, 232, 210, 218,
                202, 0, 184, 0, 2, 16, 64, 31, 0, 0, 32, 3, 0, 0, 0,
            ],
        };

        let d5 = SvcDeltaDescription {
            name: &[
                101, 110, 116, 105, 116, 121, 95, 115, 116, 97, 116, 101, 95, 112, 108, 97, 121,
                101, 114, 95, 116, 0,
            ],
            total_fields: 48,
            fields: vec![],
            clone: &[
                249, 3, 1, 0, 0, 8, 115, 75, 107, 163, 75, 107, 43, 3, 224, 2, 8, 64, 0, 125, 0, 0,
                0, 125, 0, 0, 200, 31, 1, 0, 0, 128, 153, 92, 88, 91, 25, 0, 12, 64, 0, 2, 232, 3,
                0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 223, 228, 210, 206, 210, 220, 182, 96, 186,
                0, 32, 0, 2, 36, 0, 232, 3, 0, 64, 31, 0, 0, 242, 7, 1, 0, 0, 16, 230, 118, 198,
                86, 54, 183, 5, 211, 5, 192, 1, 16, 0, 1, 250, 0, 0, 0, 250, 0, 0, 144, 63, 8, 0,
                0, 128, 48, 183, 51, 182, 178, 185, 173, 152, 46, 0, 16, 128, 0, 8, 208, 7, 0, 0,
                208, 7, 0, 128, 252, 17, 0, 0, 0, 190, 201, 165, 157, 165, 185, 109, 197, 116, 1,
                80, 0, 4, 72, 0, 208, 7, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 240, 77, 46, 237, 44,
                205, 109, 75, 166, 11, 0, 3, 32, 64, 2, 128, 62, 0, 0, 244, 1, 0, 32, 127, 8, 0, 0,
                0, 103, 97, 105, 116, 115, 101, 113, 117, 101, 110, 99, 101, 0, 184, 0, 1, 8, 160,
                15, 0, 0, 160, 15, 0, 0, 249, 67, 0, 0, 0, 152, 43, 139, 171, 43, 115, 27, 43, 3,
                96, 1, 8, 64, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 64, 219, 27, 89, 25,
                91, 154, 27, 89, 25, 30, 0, 10, 64, 128, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 16,
                0, 0, 0, 218, 222, 236, 202, 232, 242, 224, 202, 0, 176, 0, 2, 8, 64, 31, 0, 0, 64,
                31, 0, 0, 242, 39, 0, 0, 0, 48, 247, 198, 150, 70, 6, 160, 3, 16, 48, 0, 250, 0, 0,
                0, 250, 0, 0, 144, 63, 2, 0, 0, 192, 182, 52, 183, 185, 45, 152, 46, 0, 62, 128, 0,
                8, 128, 62, 0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 182, 165, 185, 205, 109, 197,
                116, 1, 0, 2, 4, 64, 0, 244, 1, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 176, 45, 205,
                109, 110, 75, 166, 11, 128, 16, 32, 0, 2, 160, 15, 0, 0, 244, 1, 0, 32, 127, 4, 0,
                0, 128, 109, 97, 120, 115, 91, 48, 93, 0, 136, 0, 1, 16, 0, 125, 0, 0, 160, 15, 0,
                0, 249, 35, 0, 0, 0, 108, 11, 195, 155, 219, 138, 233, 2, 96, 4, 8, 128, 0, 232, 3,
                0, 0, 125, 0, 0, 200, 31, 1, 0, 0, 96, 91, 24, 222, 220, 150, 76, 23, 0, 36, 64, 0,
                4, 64, 31, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 238, 202, 194, 224, 222, 220,
                218, 222, 200, 202, 216, 0, 104, 1, 2, 20, 64, 31, 0, 0, 64, 31, 0, 0, 242, 135, 0,
                0, 0, 240, 118, 231, 86, 38, 7, 128, 9, 16, 80, 0, 250, 0, 0, 0, 250, 0, 0, 144,
                63, 4, 0, 0, 128, 50, 51, 179, 178, 49, 186, 57, 0, 30, 128, 0, 4, 208, 7, 0, 0,
                208, 7, 0, 128, 252, 65, 0, 0, 0, 132, 185, 157, 177, 149, 205, 109, 201, 116, 1,
                144, 0, 4, 64, 128, 62, 0, 0, 128, 62, 0, 0, 228, 15, 1, 0, 0, 96, 236, 141, 237,
                77, 174, 45, 12, 14, 128, 6, 32, 0, 2, 244, 1, 0, 0, 244, 1, 0, 32, 127, 4, 0, 0,
                128, 102, 114, 97, 109, 101, 114, 97, 116, 101, 0, 96, 0, 1, 8, 0, 250, 0, 0, 160,
                15, 0, 0, 249, 19, 0, 0, 0, 156, 91, 75, 115, 3, 192, 1, 8, 72, 0, 125, 0, 0, 0,
                125, 0, 0, 200, 95, 0, 0, 0, 192, 216, 155, 27, 157, 220, 27, 27, 91, 153, 220, 22,
                76, 23, 0, 26, 64, 0, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 2, 0, 0, 0, 198, 222,
                220, 232, 228, 222, 216, 216, 202, 228, 182, 98, 186, 0, 210, 0, 2, 16, 64, 31, 0,
                0, 64, 31, 0, 0, 242, 23, 0, 0, 0, 48, 246, 230, 70, 39, 247, 198, 198, 86, 38,
                183, 37, 211, 5, 160, 6, 16, 128, 0, 250, 0, 0, 0, 250, 0, 0, 144, 191, 0, 0, 0,
                128, 177, 55, 55, 58, 185, 55, 54, 182, 50, 185, 173, 153, 46, 128, 53, 128, 0, 4,
                208, 7, 0, 0, 208, 7, 0, 128, 252, 5, 0, 0, 0, 136, 177, 149, 185, 145, 165, 185,
                157, 109, 193, 116, 1, 176, 1, 4, 32, 128, 62, 0, 0, 128, 62, 0, 0, 228, 47, 0, 0,
                0, 64, 140, 173, 204, 141, 44, 205, 237, 108, 43, 166, 11, 160, 13, 32, 0, 1, 244,
                1, 0, 0, 244, 1, 0, 32, 127, 8, 0, 0, 0, 98, 111, 100, 121, 0, 100, 0, 1, 8, 160,
                15, 0, 0, 160, 15, 0, 0, 249, 67, 0, 0, 0, 144, 43, 115, 35, 43, 147, 107, 123, 35,
                43, 3, 64, 2, 8, 64, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 128, 92, 153,
                27, 89, 153, 92, 88, 27, 29, 0, 19, 64, 0, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 16,
                0, 0, 0, 228, 202, 220, 200, 202, 228, 204, 240, 0, 168, 0, 2, 16, 64, 31, 0, 0,
                64, 31, 0, 0, 242, 71, 0, 0, 0, 48, 55, 22, 198, 86, 6, 0, 4, 16, 0, 1, 0, 250, 0,
                0, 250, 0, 0, 144, 191, 0, 0, 0, 0, 185, 50, 55, 178, 50, 185, 177, 55, 182, 55,
                57, 23, 57, 0, 40, 128, 0, 4, 208, 7, 0, 0, 208, 7, 0, 128, 252, 5, 0, 0, 0, 200,
                149, 185, 145, 149, 201, 141, 189, 177, 189, 201, 185, 156, 1, 68, 1, 4, 32, 128,
                62, 0, 0, 128, 62, 0, 0, 228, 47, 0, 0, 0, 64, 174, 204, 141, 172, 76, 110, 236,
                141, 237, 77, 206, 69, 12, 64, 10, 32, 0, 1, 244, 1, 0, 0, 244, 1, 0, 32, 127, 4,
                0, 0, 128, 102, 114, 105, 99, 116, 105, 111, 110, 0, 156, 0, 1, 16, 0, 125, 0, 0,
                160, 15, 0, 0, 249, 67, 0, 0, 0, 168, 155, 43, 67, 171, 99, 99, 3, 64, 6, 8, 8, 0,
                125, 0, 0, 0, 125, 0, 0, 200, 31, 1, 0, 0, 224, 153, 92, 152, 93, 26, 93, 30, 0,
                40, 64, 0, 4, 0, 125, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 194, 210, 218, 202,
                220, 232, 0, 40, 1, 2, 22, 64, 31, 0, 0, 64, 31, 0, 0, 242, 71, 0, 0, 0, 40, 22,
                54, 87, 102, 87, 198, 246, 54, 150, 70, 151, 183, 5, 211, 5, 192, 11, 16, 0, 1,
                208, 7, 0, 0, 250, 0, 0, 144, 63, 2, 0, 0, 64, 177, 176, 185, 50, 187, 50, 182,
                183, 177, 52, 186, 188, 173, 152, 46, 0, 96, 128, 0, 8, 128, 62, 0, 0, 208, 7, 0,
                128, 252, 17, 0, 0, 0, 138, 133, 205, 149, 217, 149, 177, 189, 141, 165, 209, 229,
                109, 201, 116, 1, 16, 3, 4, 64, 0, 244, 1, 0, 128, 62, 0, 0, 228, 15, 1, 0, 0, 96,
                14, 174, 108, 140, 46, 140, 238, 77, 14, 0, 22, 32, 32, 0, 244, 1, 0, 0, 244, 1, 0,
                32, 127, 8, 0, 0, 0, 105, 117, 115, 101, 114, 52, 0, 16, 1, 1, 2, 160, 15, 0, 0,
                160, 15, 0, 0,
            ],
        };

        let d6 = SvcDeltaDescription {
            name: &[
                101, 110, 116, 105, 116, 121, 95, 115, 116, 97, 116, 101, 95, 116, 0,
            ],
            total_fields: 52,
            fields: vec![],
            clone: &[
                249, 3, 1, 0, 0, 8, 115, 75, 107, 163, 75, 107, 43, 3, 224, 2, 8, 64, 0, 125, 0, 0,
                0, 125, 0, 0, 200, 31, 1, 0, 0, 128, 153, 92, 88, 91, 25, 0, 12, 64, 0, 2, 232, 3,
                0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 223, 228, 210, 206, 210, 220, 182, 96, 186,
                0, 32, 0, 2, 32, 0, 250, 0, 0, 64, 31, 0, 0, 242, 7, 1, 0, 0, 16, 230, 118, 198,
                86, 54, 183, 5, 211, 5, 192, 1, 16, 0, 1, 250, 0, 0, 0, 250, 0, 0, 144, 63, 8, 0,
                0, 128, 48, 183, 51, 182, 178, 185, 173, 152, 46, 0, 16, 128, 0, 8, 208, 7, 0, 0,
                208, 7, 0, 128, 252, 17, 0, 0, 0, 190, 201, 165, 157, 165, 185, 109, 197, 116, 1,
                80, 0, 4, 64, 0, 244, 1, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 240, 77, 46, 237, 44,
                205, 109, 75, 166, 11, 0, 3, 32, 0, 2, 160, 15, 0, 0, 244, 1, 0, 32, 127, 8, 0, 0,
                0, 115, 101, 113, 117, 101, 110, 99, 101, 0, 44, 0, 1, 8, 160, 15, 0, 0, 160, 15,
                0, 0, 249, 67, 0, 0, 0, 104, 123, 35, 43, 99, 75, 115, 35, 43, 195, 3, 64, 1, 8,
                80, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 64, 219, 155, 93, 25, 93, 30, 92,
                25, 0, 22, 64, 0, 1, 232, 3, 0, 0, 232, 3, 0, 64, 254, 4, 0, 0, 0, 230, 222, 216,
                210, 200, 0, 116, 0, 2, 6, 64, 31, 0, 0, 64, 31, 0, 0, 242, 71, 0, 0, 0, 216, 150,
                230, 54, 183, 5, 211, 5, 192, 7, 16, 0, 1, 208, 7, 0, 0, 250, 0, 0, 144, 63, 2, 0,
                0, 192, 182, 52, 183, 185, 173, 152, 46, 0, 64, 128, 0, 8, 128, 62, 0, 0, 208, 7,
                0, 128, 252, 17, 0, 0, 0, 182, 165, 185, 205, 109, 201, 116, 1, 16, 2, 4, 64, 0,
                244, 1, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 176, 45, 12, 111, 110, 11, 166, 11, 0,
                17, 32, 0, 2, 160, 15, 0, 0, 244, 1, 0, 32, 127, 4, 0, 0, 128, 109, 97, 120, 115,
                91, 49, 93, 0, 140, 0, 1, 16, 0, 125, 0, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0, 108,
                11, 195, 155, 219, 146, 233, 2, 128, 4, 8, 128, 0, 232, 3, 0, 0, 125, 0, 0, 200,
                31, 1, 0, 0, 96, 153, 27, 25, 220, 219, 220, 22, 76, 23, 0, 60, 64, 64, 3, 232, 3,
                0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 203, 220, 200, 224, 222, 230, 182, 98, 186,
                0, 232, 1, 2, 26, 64, 31, 0, 0, 64, 31, 0, 0, 242, 71, 0, 0, 0, 88, 230, 70, 6,
                247, 54, 183, 37, 211, 5, 128, 15, 16, 208, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63, 2,
                0, 0, 192, 57, 186, 48, 57, 58, 184, 183, 185, 45, 152, 46, 0, 114, 128, 128, 6,
                208, 7, 0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 206, 209, 133, 201, 209, 193, 189,
                205, 109, 197, 116, 1, 160, 3, 4, 52, 128, 62, 0, 0, 128, 62, 0, 0, 228, 143, 0, 0,
                0, 112, 142, 46, 76, 142, 14, 238, 109, 110, 75, 166, 11, 128, 29, 32, 160, 1, 244,
                1, 0, 0, 244, 1, 0, 32, 127, 64, 0, 0, 0, 105, 109, 112, 97, 99, 116, 116, 105,
                109, 101, 0, 252, 0, 1, 13, 128, 26, 6, 0, 160, 15, 0, 0, 249, 3, 2, 0, 0, 152,
                163, 11, 147, 163, 163, 75, 107, 43, 3, 0, 8, 8, 104, 0, 212, 48, 0, 0, 125, 0, 0,
                200, 31, 2, 0, 0, 192, 93, 89, 24, 220, 155, 91, 219, 27, 89, 25, 27, 0, 45, 64,
                128, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 222, 238, 220, 202, 228, 0,
                48, 1, 2, 10, 64, 31, 0, 0, 64, 31, 0, 0, 242, 135, 0, 0, 0, 80, 102, 102, 86, 54,
                70, 55, 7, 192, 3, 16, 128, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63, 4, 0, 0, 128, 50,
                51, 182, 176, 179, 57, 0, 34, 128, 128, 0, 208, 7, 0, 0, 208, 7, 0, 128, 252, 65,
                0, 0, 0, 132, 185, 157, 177, 149, 205, 109, 201, 116, 1, 144, 0, 4, 64, 128, 62, 0,
                0, 128, 62, 0, 0, 228, 15, 1, 0, 0, 96, 236, 141, 237, 77, 174, 45, 12, 14, 128, 6,
                32, 0, 2, 244, 1, 0, 0, 244, 1, 0, 32, 127, 4, 0, 0, 128, 102, 114, 97, 109, 101,
                114, 97, 116, 101, 0, 96, 0, 1, 8, 0, 250, 0, 0, 160, 15, 0, 0, 249, 19, 0, 0, 0,
                156, 91, 75, 115, 3, 192, 1, 8, 72, 0, 125, 0, 0, 0, 125, 0, 0, 200, 95, 0, 0, 0,
                192, 216, 155, 27, 157, 220, 27, 27, 91, 153, 220, 22, 76, 23, 0, 26, 64, 0, 2,
                232, 3, 0, 0, 232, 3, 0, 64, 254, 2, 0, 0, 0, 198, 222, 220, 232, 228, 222, 216,
                216, 202, 228, 182, 98, 186, 0, 210, 0, 2, 16, 64, 31, 0, 0, 64, 31, 0, 0, 242, 23,
                0, 0, 0, 48, 246, 230, 70, 39, 247, 198, 198, 86, 38, 183, 37, 211, 5, 160, 6, 16,
                128, 0, 250, 0, 0, 0, 250, 0, 0, 144, 191, 0, 0, 0, 128, 177, 55, 55, 58, 185, 55,
                54, 182, 50, 185, 173, 153, 46, 128, 53, 128, 0, 4, 208, 7, 0, 0, 208, 7, 0, 128,
                252, 5, 0, 0, 0, 136, 177, 149, 185, 145, 165, 185, 157, 109, 193, 116, 1, 176, 1,
                4, 32, 128, 62, 0, 0, 128, 62, 0, 0, 228, 47, 0, 0, 0, 64, 140, 173, 204, 141, 44,
                205, 237, 108, 43, 166, 11, 160, 13, 32, 0, 1, 244, 1, 0, 0, 244, 1, 0, 32, 127, 8,
                0, 0, 0, 98, 111, 100, 121, 0, 100, 0, 1, 8, 160, 15, 0, 0, 160, 15, 0, 0, 249, 67,
                0, 0, 0, 144, 43, 115, 35, 43, 147, 107, 123, 35, 43, 3, 64, 2, 8, 64, 0, 125, 0,
                0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 128, 92, 153, 27, 89, 153, 92, 88, 27, 29, 0,
                19, 64, 0, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 228, 202, 220, 200,
                202, 228, 204, 240, 0, 168, 0, 2, 16, 64, 31, 0, 0, 64, 31, 0, 0, 242, 71, 0, 0, 0,
                48, 55, 22, 198, 86, 6, 0, 4, 16, 0, 1, 0, 250, 0, 0, 250, 0, 0, 144, 191, 0, 0, 0,
                0, 185, 50, 55, 178, 50, 185, 177, 55, 182, 55, 57, 23, 57, 0, 40, 128, 0, 4, 208,
                7, 0, 0, 208, 7, 0, 128, 252, 5, 0, 0, 0, 200, 149, 185, 145, 149, 201, 141, 189,
                177, 189, 201, 185, 156, 1, 68, 1, 4, 32, 128, 62, 0, 0, 128, 62, 0, 0, 228, 47, 0,
                0, 0, 64, 174, 204, 141, 172, 76, 110, 236, 141, 237, 77, 206, 69, 12, 64, 10, 32,
                0, 1, 244, 1, 0, 0, 244, 1, 0, 32, 127, 8, 0, 0, 0, 97, 105, 109, 101, 110, 116, 0,
                148, 0, 1, 11, 160, 15, 0, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0, 20, 11, 155, 43,
                179, 43, 99, 123, 27, 75, 163, 203, 219, 130, 233, 2, 224, 5, 8, 128, 0, 232, 3, 0,
                0, 125, 0, 0, 200, 31, 1, 0, 0, 160, 88, 216, 92, 153, 93, 25, 219, 219, 88, 26,
                93, 222, 86, 76, 23, 0, 48, 64, 0, 4, 64, 31, 0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0,
                197, 194, 230, 202, 236, 202, 216, 222, 198, 210, 232, 242, 182, 100, 186, 0, 136,
                1, 2, 32, 0, 250, 0, 0, 64, 31, 0, 0, 242, 135, 0, 0, 0, 144, 86, 55, 87, 38, 71,
                3, 0, 17, 16, 32, 0, 250, 0, 0, 0, 250, 0, 0, 0,
            ],
        };

        let d7 = SvcDeltaDescription {
            name: &[99, 108, 105, 101, 110, 116, 100, 97, 116, 97, 95, 116, 0],
            total_fields: 47,
            fields: vec![],
            clone: &[
                249, 67, 0, 0, 0, 48, 99, 163, 74, 107, 43, 155, 162, 43, 131, 155, 122, 171, 115,
                35, 3, 96, 2, 8, 80, 0, 125, 0, 0, 0, 125, 0, 0, 200, 30, 1, 0, 0, 224, 155, 92,
                218, 89, 154, 219, 22, 76, 23, 64, 64, 5, 0, 244, 1, 0, 232, 3, 0, 64, 254, 8, 0,
                0, 0, 223, 228, 210, 206, 210, 220, 182, 98, 186, 0, 8, 0, 2, 42, 0, 160, 15, 0,
                64, 31, 0, 0, 242, 71, 0, 0, 0, 104, 87, 198, 246, 54, 150, 70, 151, 183, 5, 211,
                5, 192, 0, 16, 0, 1, 208, 7, 0, 0, 250, 0, 0, 144, 63, 2, 0, 0, 64, 187, 50, 182,
                183, 177, 52, 186, 188, 173, 152, 46, 0, 8, 128, 0, 8, 128, 62, 0, 0, 208, 7, 0,
                128, 252, 17, 0, 0, 0, 182, 125, 153, 177, 57, 149, 225, 209, 5, 209, 209, 133,
                141, 173, 1, 240, 1, 4, 88, 0, 36, 244, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 240,
                77, 46, 237, 44, 205, 109, 75, 166, 11, 0, 1, 32, 160, 2, 0, 250, 0, 0, 244, 1, 0,
                32, 127, 4, 0, 0, 128, 118, 101, 108, 111, 99, 105, 116, 121, 91, 50, 93, 0, 20, 0,
                1, 16, 0, 125, 0, 0, 160, 15, 0, 0, 249, 67, 0, 0, 0, 12, 107, 107, 123, 251, 114,
                11, 75, 99, 155, 3, 128, 3, 8, 80, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0,
                96, 88, 91, 219, 219, 215, 28, 90, 25, 27, 219, 28, 0, 27, 64, 128, 2, 232, 3, 0,
                0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 195, 218, 218, 222, 190, 198, 202, 216, 216,
                230, 0, 232, 0, 2, 20, 64, 31, 0, 0, 64, 31, 0, 0, 242, 135, 0, 0, 0, 24, 214, 214,
                246, 246, 37, 247, 54, 182, 86, 70, 55, 7, 128, 7, 16, 160, 0, 250, 0, 0, 0, 250,
                0, 0, 144, 63, 4, 0, 0, 128, 182, 175, 180, 36, 50, 0, 52, 128, 128, 2, 208, 7, 0,
                0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 194, 213, 185, 141, 161, 133, 185, 157, 177,
                149, 109, 201, 116, 1, 144, 0, 4, 84, 0, 244, 1, 0, 128, 62, 0, 0, 228, 15, 1, 0,
                0, 192, 140, 45, 236, 108, 14, 0, 5, 32, 0, 4, 244, 1, 0, 0, 244, 1, 0, 32, 127, 8,
                0, 0, 0, 119, 101, 97, 112, 111, 110, 97, 110, 105, 109, 0, 100, 0, 1, 8, 160, 15,
                0, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0, 68, 43, 11, 99, 163, 67, 3, 0, 2, 8, 80, 0,
                125, 0, 0, 0, 125, 0, 0, 200, 31, 1, 0, 0, 64, 91, 24, 222, 28, 92, 89, 25, 25, 0,
                23, 64, 0, 4, 16, 39, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 204, 216, 136, 234,
                198, 214, 168, 210, 218, 202, 0, 160, 0, 2, 20, 64, 31, 0, 0, 64, 31, 0, 0, 242,
                71, 0, 0, 0, 104, 151, 86, 118, 247, 245, 102, 54, 183, 37, 211, 5, 192, 3, 16,
                160, 0, 232, 3, 0, 0, 250, 0, 0, 144, 63, 2, 0, 0, 64, 184, 58, 183, 49, 180, 48,
                183, 51, 182, 178, 45, 152, 46, 0, 14, 128, 0, 13, 0, 0, 250, 0, 208, 7, 0, 128,
                252, 17, 0, 0, 0, 194, 213, 185, 141, 161, 133, 185, 157, 177, 149, 109, 197, 116,
                1, 128, 0, 4, 104, 0, 0, 208, 7, 128, 62, 0, 0, 228, 15, 1, 0, 0, 192, 46, 173,
                236, 174, 237, 141, 172, 140, 13, 0, 3, 32, 64, 1, 244, 1, 0, 0, 244, 1, 0, 32,
                127, 8, 0, 0, 0, 119, 101, 97, 112, 111, 110, 115, 0, 72, 0, 1, 32, 160, 15, 0, 0,
                160, 15, 0, 0, 249, 67, 0, 0, 0, 128, 171, 155, 67, 107, 155, 43, 27, 3, 32, 4, 8,
                88, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 0, 89, 89, 24, 153, 25, 91, 216,
                25, 0, 34, 64, 192, 0, 232, 3, 0, 0, 232, 3, 0, 64, 254, 8, 0, 0, 0, 204, 222, 236,
                0, 192, 0, 2, 16, 64, 31, 0, 0, 64, 31, 0, 0, 242, 7, 8, 0, 0, 0, 135, 150, 55,
                151, 230, 102, 246, 6, 192, 8, 16, 16, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63, 4, 0,
                0, 0, 177, 36, 55, 162, 186, 177, 53, 0, 34, 128, 128, 0, 208, 7, 0, 0, 208, 7, 0,
                128, 252, 33, 0, 0, 0, 152, 177, 77, 221, 165, 181, 81, 165, 181, 149, 1, 80, 1, 4,
                40, 128, 62, 0, 0, 128, 62, 0, 0, 228, 15, 1, 0, 0, 224, 46, 140, 174, 76, 78, 173,
                174, 13, 142, 46, 173, 173, 12, 0, 11, 32, 224, 1, 244, 1, 0, 0, 244, 1, 0, 32,
                127, 8, 0, 0, 0, 119, 97, 116, 101, 114, 108, 101, 118, 101, 108, 0, 44, 0, 1, 2,
                160, 15, 0, 0, 160, 15, 0, 0, 249, 67, 0, 0, 0, 72, 171, 155, 43, 147, 139, 1, 96,
                12, 8, 24, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 2, 0, 0, 64, 90, 221, 92, 153, 156,
                12, 0, 100, 64, 128, 1, 232, 3, 0, 0, 232, 3, 0, 64, 254, 16, 0, 0, 0, 210, 234,
                230, 202, 228, 102, 0, 40, 3, 2, 16, 64, 31, 0, 0, 64, 31, 0, 0, 242, 135, 0, 0, 0,
                144, 86, 55, 87, 38, 71, 3, 128, 25, 16, 32, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63,
                2, 0, 0, 0, 187, 186, 185, 50, 57, 153, 45, 152, 46, 0, 220, 128, 128, 4, 208, 7,
                0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 216, 213, 205, 149, 201, 201, 108, 197,
                116, 1, 240, 6, 4, 36, 128, 62, 0, 0, 128, 62, 0, 0, 228, 143, 0, 0, 0, 192, 174,
                110, 174, 76, 78, 102, 75, 166, 11, 0, 56, 32, 32, 1, 244, 1, 0, 0, 244, 1, 0, 32,
                127, 4, 0, 0, 0, 118, 117, 115, 101, 114, 51, 91, 48, 93, 0, 196, 1, 1, 9, 160, 15,
                0, 0, 160, 15, 0, 0, 249, 35, 0, 0, 0, 176, 171, 155, 43, 147, 155, 217, 138, 233,
                2, 64, 14, 8, 72, 0, 125, 0, 0, 0, 125, 0, 0, 200, 31, 1, 0, 0, 128, 93, 221, 92,
                153, 220, 204, 150, 76, 23, 0, 115, 64, 64, 2, 232, 3, 0, 0, 232, 3, 0, 64, 254, 8,
                0, 0, 0, 236, 234, 230, 202, 228, 104, 182, 96, 186, 0, 160, 3, 2, 18, 64, 31, 0,
                0, 64, 31, 0, 0, 242, 71, 0, 0, 0, 96, 87, 55, 87, 38, 71, 179, 21, 211, 5, 64, 29,
                16, 144, 0, 250, 0, 0, 0, 250, 0, 0, 144, 63, 2, 0, 0, 0, 179, 186, 185, 50, 185,
                24, 0, 206, 128, 128, 4, 208, 7, 0, 0, 208, 7, 0, 128, 252, 17, 0, 0, 0, 152, 213,
                205, 149, 201, 201, 0, 128, 6, 4, 56, 128, 62, 0, 0, 128, 62, 0, 0, 228, 143, 0, 0,
                0, 192, 172, 110, 174, 76, 110, 6, 128, 52, 32, 64, 1, 244, 1, 0, 0, 244, 1, 0, 0,
            ],
        };

        [d1, d2, d3, d4, d5, d6, d7]
    }};
}

#[macro_export]
macro_rules! get_cs_delta_decoder_table {
    () => {{
        use demosuperimpose_goldsrc::types::DeltaDecoderS;

        let mut dt = DeltaDecoderTable::new();

        dt.insert(
            "event_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![101, 110, 116, 105, 110, 100, 101, 120, 0],
                    bits: 11,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![98, 112, 97, 114, 97, 109, 49, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![98, 112, 97, 114, 97, 109, 50, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 48, 93, 0],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 49, 93, 0],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 50, 93, 0],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![102, 112, 97, 114, 97, 109, 49, 0],
                    bits: 20,
                    divisor: 100.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![102, 112, 97, 114, 97, 109, 50, 0],
                    bits: 20,
                    divisor: 100.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![105, 112, 97, 114, 97, 109, 49, 0],
                    bits: 18,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![105, 112, 97, 114, 97, 109, 50, 0],
                    bits: 18,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 48, 93, 0],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 49, 93, 0],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 50, 93, 0],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![100, 117, 99, 107, 105, 110, 103, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
            ],
        );

        dt.insert(
            "weapon_data_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 108, 84, 105, 109, 101, 87, 101, 97, 112, 111, 110, 73, 100,
                        108, 101, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 108, 78, 101, 120, 116, 80, 114, 105, 109, 97, 114, 121, 65,
                        116, 116, 97, 99, 107, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 108, 78, 101, 120, 116, 82, 101, 108, 111, 97, 100, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 78, 101, 120, 116, 65, 105, 109, 66, 111, 110, 117, 115, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 108, 78, 101, 120, 116, 83, 101, 99, 111, 110, 100, 97, 114,
                        121, 65, 116, 116, 97, 99, 107, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 95, 105, 67, 108, 105, 112, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![109, 95, 102, 108, 80, 117, 109, 112, 84, 105, 109, 101, 0],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 73, 110, 83, 112, 101, 99, 105, 97, 108, 82, 101, 108, 111,
                        97, 100, 0,
                    ],
                    bits: 2,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 82, 101, 108, 111, 97, 100, 84, 105, 109, 101, 0,
                    ],
                    bits: 16,
                    divisor: 100.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![109, 95, 102, 73, 110, 82, 101, 108, 111, 97, 100, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 65, 105, 109, 101, 100, 68, 97, 109, 97, 103, 101, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![109, 95, 102, 73, 110, 90, 111, 111, 109, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 105, 87, 101, 97, 112, 111, 110, 83, 116, 97, 116, 101, 0,
                    ],
                    bits: 7,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![109, 95, 105, 73, 100, 0],
                    bits: 5,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![102, 117, 115, 101, 114, 49, 0],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![102, 117, 115, 101, 114, 50, 0],
                    bits: 22,
                    divisor: 128.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![102, 117, 115, 101, 114, 51, 0],
                    bits: 22,
                    divisor: 128.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 49, 0],
                    bits: 16,
                    divisor: 128.0,
                    flags: 2147483656,
                },
            ],
        );
        dt.insert(
            "usercmd_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![108, 101, 114, 112, 95, 109, 115, 101, 99, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 2,
                },
                DeltaDecoderS {
                    name: vec![109, 115, 101, 99, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        118, 105, 101, 119, 97, 110, 103, 108, 101, 115, 91, 49, 93, 0,
                    ],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![
                        118, 105, 101, 119, 97, 110, 103, 108, 101, 115, 91, 48, 93, 0,
                    ],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![98, 117, 116, 116, 111, 110, 115, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 2,
                },
                DeltaDecoderS {
                    name: vec![102, 111, 114, 119, 97, 114, 100, 109, 111, 118, 101, 0],
                    bits: 12,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![108, 105, 103, 104, 116, 108, 101, 118, 101, 108, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![115, 105, 100, 101, 109, 111, 118, 101, 0],
                    bits: 12,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![117, 112, 109, 111, 118, 101, 0],
                    bits: 12,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![105, 109, 112, 117, 108, 115, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        118, 105, 101, 119, 97, 110, 103, 108, 101, 115, 91, 50, 93, 0,
                    ],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![105, 109, 112, 97, 99, 116, 95, 105, 110, 100, 101, 120, 0],
                    bits: 6,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        105, 109, 112, 97, 99, 116, 95, 112, 111, 115, 105, 116, 105, 111, 110, 91,
                        48, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        105, 109, 112, 97, 99, 116, 95, 112, 111, 115, 105, 116, 105, 111, 110, 91,
                        49, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        105, 109, 112, 97, 99, 116, 95, 112, 111, 115, 105, 116, 105, 111, 110, 91,
                        50, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
            ],
        );
        dt.insert(
            "clientdata_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![
                        102, 108, 84, 105, 109, 101, 83, 116, 101, 112, 83, 111, 117, 110, 100, 0,
                    ],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 48, 93, 0],
                    bits: 21,
                    divisor: 128.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 49, 93, 0],
                    bits: 21,
                    divisor: 128.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![118, 101, 108, 111, 99, 105, 116, 121, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![118, 101, 108, 111, 99, 105, 116, 121, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        109, 95, 102, 108, 78, 101, 120, 116, 65, 116, 116, 97, 99, 107, 0,
                    ],
                    bits: 22,
                    divisor: 1000.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 50, 93, 0],
                    bits: 21,
                    divisor: 128.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![118, 101, 108, 111, 99, 105, 116, 121, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 109, 109, 111, 95, 110, 97, 105, 108, 115, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![97, 109, 109, 111, 95, 115, 104, 101, 108, 108, 115, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![97, 109, 109, 111, 95, 99, 101, 108, 108, 115, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![97, 109, 109, 111, 95, 114, 111, 99, 107, 101, 116, 115, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 2147483656,
                },
                DeltaDecoderS {
                    name: vec![109, 95, 105, 73, 100, 0],
                    bits: 5,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        112, 117, 110, 99, 104, 97, 110, 103, 108, 101, 91, 50, 93, 0,
                    ],
                    bits: 21,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![102, 108, 97, 103, 115, 0],
                    bits: 32,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![119, 101, 97, 112, 111, 110, 97, 110, 105, 109, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![104, 101, 97, 108, 116, 104, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 112, 101, 101, 100, 0],
                    bits: 16,
                    divisor: 10.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![102, 108, 68, 117, 99, 107, 84, 105, 109, 101, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![118, 105, 101, 119, 95, 111, 102, 115, 91, 50, 93, 0],
                    bits: 10,
                    divisor: 4.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        112, 117, 110, 99, 104, 97, 110, 103, 108, 101, 91, 48, 93, 0,
                    ],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        112, 117, 110, 99, 104, 97, 110, 103, 108, 101, 91, 49, 93, 0,
                    ],
                    bits: 26,
                    divisor: 8192.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![118, 105, 101, 119, 109, 111, 100, 101, 108, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![119, 101, 97, 112, 111, 110, 115, 0],
                    bits: 32,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![112, 117, 115, 104, 109, 115, 101, 99, 0],
                    bits: 11,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![100, 101, 97, 100, 102, 108, 97, 103, 0],
                    bits: 3,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![102, 111, 118, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![112, 104, 121, 115, 105, 110, 102, 111, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 128,
                },
                DeltaDecoderS {
                    name: vec![98, 73, 110, 68, 117, 99, 107, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![102, 108, 83, 119, 105, 109, 84, 105, 109, 101, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        119, 97, 116, 101, 114, 106, 117, 109, 112, 116, 105, 109, 101, 0,
                    ],
                    bits: 15,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![119, 97, 116, 101, 114, 108, 101, 118, 101, 108, 0],
                    bits: 2,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 49, 0],
                    bits: 3,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 50, 0],
                    bits: 6,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 51, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 52, 0],
                    bits: 2,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 50, 91, 48, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 50, 91, 49, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 50, 91, 50, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 51, 91, 48, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 51, 91, 49, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 51, 91, 50, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 52, 91, 48, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![118, 117, 115, 101, 114, 52, 91, 49, 93, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![102, 117, 115, 101, 114, 49, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![102, 117, 115, 101, 114, 50, 0],
                    bits: 14,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![102, 117, 115, 101, 114, 51, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 4,
                },
            ],
        );
        dt.insert(
            "entity_state_player_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![97, 110, 105, 109, 116, 105, 109, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 32,
                },
                DeltaDecoderS {
                    name: vec![102, 114, 97, 109, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 48, 93, 0],
                    bits: 18,
                    divisor: 32.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 49, 93, 0],
                    bits: 18,
                    divisor: 32.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 50, 93, 0],
                    bits: 18,
                    divisor: 32.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![103, 97, 105, 116, 115, 101, 113, 117, 101, 110, 99, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 101, 113, 117, 101, 110, 99, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![109, 111, 100, 101, 108, 105, 110, 100, 101, 120, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![109, 111, 118, 101, 116, 121, 112, 101, 0],
                    bits: 4,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 111, 108, 105, 100, 0],
                    bits: 3,
                    divisor: 1.0,
                    flags: 2,
                },
                DeltaDecoderS {
                    name: vec![109, 105, 110, 115, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 105, 110, 115, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 105, 110, 115, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![119, 101, 97, 112, 111, 110, 109, 111, 100, 101, 108, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![111, 119, 110, 101, 114, 0],
                    bits: 5,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![101, 102, 102, 101, 99, 116, 115, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![99, 111, 108, 111, 114, 109, 97, 112, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![102, 114, 97, 109, 101, 114, 97, 116, 101, 0],
                    bits: 8,
                    divisor: 16.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 107, 105, 110, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 2147483650,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 48, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 49, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 50, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 51, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![98, 108, 101, 110, 100, 105, 110, 103, 91, 48, 93, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![98, 108, 101, 110, 100, 105, 110, 103, 91, 49, 93, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![98, 111, 100, 121, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 109, 111, 100, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 97, 109, 116, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 102, 120, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 99, 97, 108, 101, 0],
                    bits: 16,
                    divisor: 256.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 114, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 103, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 98, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![102, 114, 105, 99, 116, 105, 111, 110, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![117, 115, 101, 104, 117, 108, 108, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![103, 114, 97, 118, 105, 116, 121, 0],
                    bits: 16,
                    divisor: 32.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 105, 109, 101, 110, 116, 0],
                    bits: 11,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        98, 97, 115, 101, 118, 101, 108, 111, 99, 105, 116, 121, 91, 48, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        98, 97, 115, 101, 118, 101, 108, 111, 99, 105, 116, 121, 91, 49, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        98, 97, 115, 101, 118, 101, 108, 111, 99, 105, 116, 121, 91, 50, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 112, 101, 99, 116, 97, 116, 111, 114, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 52, 0],
                    bits: 2,
                    divisor: 1.0,
                    flags: 8,
                },
            ],
        );
        dt.insert(
            "delta_description_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![102, 108, 97, 103, 115],
                    bits: 32,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![110, 97, 109, 101],
                    bits: 8,
                    divisor: 1.0,
                    flags: 128,
                },
                DeltaDecoderS {
                    name: vec![111, 102, 102, 115, 101, 116],
                    bits: 16,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 105, 122, 101],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![98, 105, 116, 115],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![100, 105, 118, 105, 115, 111, 114],
                    bits: 32,
                    divisor: 4000.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![
                        112, 114, 101, 77, 117, 108, 116, 105, 112, 108, 105, 101, 114,
                    ],
                    bits: 32,
                    divisor: 4000.0,
                    flags: 4,
                },
            ],
        );
        dt.insert(
            "entity_state_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![97, 110, 105, 109, 116, 105, 109, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 32,
                },
                DeltaDecoderS {
                    name: vec![102, 114, 97, 109, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 101, 113, 117, 101, 110, 99, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![109, 111, 100, 101, 108, 105, 110, 100, 101, 120, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![109, 111, 118, 101, 116, 121, 112, 101, 0],
                    bits: 4,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 111, 108, 105, 100, 0],
                    bits: 3,
                    divisor: 1.0,
                    flags: 2,
                },
                DeltaDecoderS {
                    name: vec![109, 105, 110, 115, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 105, 110, 115, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 105, 110, 115, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 91, 48, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 91, 49, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![109, 97, 120, 115, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![101, 110, 100, 112, 111, 115, 91, 48, 93, 0],
                    bits: 13,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![101, 110, 100, 112, 111, 115, 91, 49, 93, 0],
                    bits: 13,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![101, 110, 100, 112, 111, 115, 91, 50, 93, 0],
                    bits: 13,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 116, 97, 114, 116, 112, 111, 115, 91, 48, 93, 0],
                    bits: 13,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 116, 97, 114, 116, 112, 111, 115, 91, 49, 93, 0],
                    bits: 13,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 116, 97, 114, 116, 112, 111, 115, 91, 50, 93, 0],
                    bits: 13,
                    divisor: 1.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![105, 109, 112, 97, 99, 116, 116, 105, 109, 101, 0],
                    bits: 13,
                    divisor: 100.0,
                    flags: 64,
                },
                DeltaDecoderS {
                    name: vec![115, 116, 97, 114, 116, 116, 105, 109, 101, 0],
                    bits: 13,
                    divisor: 100.0,
                    flags: 64,
                },
                DeltaDecoderS {
                    name: vec![119, 101, 97, 112, 111, 110, 109, 111, 100, 101, 108, 0],
                    bits: 10,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![111, 119, 110, 101, 114, 0],
                    bits: 5,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![101, 102, 102, 101, 99, 116, 115, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![101, 102, 108, 97, 103, 115, 0],
                    bits: 1,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 50, 93, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 16,
                },
                DeltaDecoderS {
                    name: vec![99, 111, 108, 111, 114, 109, 97, 112, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![102, 114, 97, 109, 101, 114, 97, 116, 101, 0],
                    bits: 8,
                    divisor: 16.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 107, 105, 110, 0],
                    bits: 9,
                    divisor: 1.0,
                    flags: 2147483650,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 48, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 49, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 50, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 91, 51, 93, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![98, 108, 101, 110, 100, 105, 110, 103, 91, 48, 93, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![98, 108, 101, 110, 100, 105, 110, 103, 91, 49, 93, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![98, 111, 100, 121, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 109, 111, 100, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 97, 109, 116, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 102, 120, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 99, 97, 108, 101, 0],
                    bits: 16,
                    divisor: 256.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 114, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 103, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 98, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![97, 105, 109, 101, 110, 116, 0],
                    bits: 11,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        98, 97, 115, 101, 118, 101, 108, 111, 99, 105, 116, 121, 91, 48, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        98, 97, 115, 101, 118, 101, 108, 111, 99, 105, 116, 121, 91, 49, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![
                        98, 97, 115, 101, 118, 101, 108, 111, 99, 105, 116, 121, 91, 50, 93, 0,
                    ],
                    bits: 16,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![105, 117, 115, 101, 114, 52, 0],
                    bits: 2,
                    divisor: 1.0,
                    flags: 8,
                },
            ],
        );
        dt.insert(
            "custom_entity_state_t\0".to_owned(),
            vec![
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 109, 111, 100, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 48, 93, 0],
                    bits: 17,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 49, 93, 0],
                    bits: 17,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![111, 114, 105, 103, 105, 110, 91, 50, 93, 0],
                    bits: 17,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 48, 93, 0],
                    bits: 17,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 49, 93, 0],
                    bits: 17,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 103, 108, 101, 115, 91, 50, 93, 0],
                    bits: 17,
                    divisor: 8.0,
                    flags: 2147483652,
                },
                DeltaDecoderS {
                    name: vec![115, 101, 113, 117, 101, 110, 99, 101, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 107, 105, 110, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![109, 111, 100, 101, 108, 105, 110, 100, 101, 120, 0],
                    bits: 16,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![115, 99, 97, 108, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![98, 111, 100, 121, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 114, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 103, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![
                        114, 101, 110, 100, 101, 114, 99, 111, 108, 111, 114, 46, 98, 0,
                    ],
                    bits: 8,
                    divisor: 1.0,
                    flags: 1,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 102, 120, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![114, 101, 110, 100, 101, 114, 97, 109, 116, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 8,
                },
                DeltaDecoderS {
                    name: vec![102, 114, 97, 109, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 4,
                },
                DeltaDecoderS {
                    name: vec![97, 110, 105, 109, 116, 105, 109, 101, 0],
                    bits: 8,
                    divisor: 1.0,
                    flags: 4,
                },
            ],
        );

        dt
    }};
}
