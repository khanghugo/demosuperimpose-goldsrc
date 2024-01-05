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
        let mut writer = BitWriter::new();
        writer.append_u32_range($num as u32, $bit);
        writer.data
    }};
}

#[macro_export]
macro_rules! init_parse {
    ($demo:ident) => {{
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
