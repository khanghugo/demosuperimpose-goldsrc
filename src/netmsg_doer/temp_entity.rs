use super::*;

macro_rules! wrap_ent {
    ($ent:ident, $data:ident) => {{
        TempEntityEntity::$ent($data)
    }};
}

pub struct TempEntity {}
impl<'a> NetMsgDoer<'a, SvcTempEntity<'a>> for TempEntity {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcTempEntity<'a>> {
        let (i, entity_type) = le_u8(i)?;

        let (i, entity) = match entity_type {
            0 => map(
                tuple((
                    count(le_i16, 3),
                    count(le_i16, 3),
                    le_i16,
                    le_u8,
                    le_u8,
                    le_u8,
                    le_u8,
                    le_u8,
                    take(4usize),
                    le_u8,
                )),
                |(
                    start_position,
                    end_position,
                    sprite_index,
                    start_frame,
                    frame_rate,
                    life,
                    width,
                    noise,
                    color,
                    speed,
                )| {
                    let res = TeBeamPoints {
                        start_position,
                        end_position,
                        sprite_index,
                        start_frame,
                        frame_rate,
                        life,
                        width,
                        noise,
                        color,
                        speed,
                    };
                    wrap_ent!(TeBeamPoints, res)
                },
            )(i)?,
            1 => map(take(20usize), |res| TempEntityEntity::TeBeamEntPoint(res))(i)?,
            2 => map(take(6usize), |res| TempEntityEntity::TeGunshot(res))(i)?,
            // The docs say 6 but its parser says 11.
            3 => map(take(11usize), |res| TempEntityEntity::TeExplosion(res))(i)?,
            4 => map(take(6usize), |res| TempEntityEntity::TeTarExplosion(res))(i)?,
            5 => map(take(10usize), |res| TempEntityEntity::TeSmoke(res))(i)?,
            6 => map(take(12usize), |res| TempEntityEntity::TeTracer(res))(i)?,
            7 => map(take(17usize), |res| TempEntityEntity::TeLightning(res))(i)?,
            8 => map(take(16usize), |res| TempEntityEntity::TeBeamEnts(res))(i)?,
            9 => map(take(6usize), |res| TempEntityEntity::TeSparks(res))(i)?,
            10 => map(take(6usize), |res| TempEntityEntity::TeLavaSplash(res))(i)?,
            11 => map(take(6usize), |res| TempEntityEntity::TeTeleport(res))(i)?,
            12 => map(take(8usize), |res| TempEntityEntity::TeExplosion2(res))(i)?,
            13 => {
                let (i, unknown1) = take(8usize)(i)?;
                let (i, entity_index) = le_i16(i)?;
                let (i, unknown2) = if entity_index != 0 {
                    map(take(2usize), |i| Some(i))(i)?
                } else {
                    (i, None)
                };

                (
                    i,
                    TempEntityEntity::TeBspDecal(TeBspDecal {
                        unknown1,
                        entity_index,
                        unknown2,
                    }),
                )
            }
            14 => map(take(9usize), |res| TempEntityEntity::TeImplosion(res))(i)?,
            15 => map(take(19usize), |res| TempEntityEntity::TeSpriteTrail(res))(i)?,
            16 => map(take(10usize), |res| TempEntityEntity::TeSprite(res))(i)?,
            18 => map(take(16usize), |res| TempEntityEntity::TeBeamSprite(res))(i)?,
            19 => map(take(24usize), |res| TempEntityEntity::TeBeamTorus(res))(i)?,
            20 => map(take(24usize), |res| TempEntityEntity::TeBeamDisk(res))(i)?,
            21 => map(take(24usize), |res| TempEntityEntity::TeBeamCylinder(res))(i)?,
            22 => map(take(10usize), |res| TempEntityEntity::TeBeamFollow(res))(i)?,
            23 => map(take(11usize), |res| TempEntityEntity::TeGlowSprite(res))(i)?,
            24 => map(take(16usize), |res| TempEntityEntity::TeBeamRing(res))(i)?,
            25 => map(take(19usize), |res| TempEntityEntity::TeStreakSplash(res))(i)?,
            27 => map(take(12usize), |res| TempEntityEntity::TeDLight(res))(i)?,
            28 => map(take(16usize), |res| TempEntityEntity::TeELight(res))(i)?,
            29 => {
                let (
                    i,
                    (
                        channel,
                        x,
                        y,
                        effect,
                        text_color,
                        effect_color,
                        fade_in_time,
                        fade_out_time,
                        hold_time,
                    ),
                ) = tuple((
                    le_i8,
                    le_i16,
                    le_i16,
                    le_i8,
                    take(4usize),
                    take(4usize),
                    le_i16,
                    le_i16,
                    le_i16,
                ))(i)?;

                let (i, effect_time) = if effect != 0 {
                    map(le_i16, |i| Some(i))(i)?
                } else {
                    (i, None)
                };

                let (i, message) = null_string(i)?;

                (
                    i,
                    TempEntityEntity::TeTextMessage(TeTextMessage {
                        channel,
                        x,
                        y,
                        effect,
                        text_color,
                        effect_color,
                        fade_in_time,
                        fade_out_time,
                        hold_time,
                        effect_time,
                        message,
                    }),
                )
            }
            30 => map(take(17usize), |res| TempEntityEntity::TeLine(res))(i)?,
            31 => map(take(17usize), |res| TempEntityEntity::TeBox(res))(i)?,
            99 => map(take(2usize), |res| TempEntityEntity::TeKillBeam(res))(i)?,
            100 => map(take(10usize), |res| TempEntityEntity::TeLargeFunnel(res))(i)?,
            101 => map(take(14usize), |res| TempEntityEntity::TeBloodStream(res))(i)?,
            102 => map(take(12usize), |res| TempEntityEntity::TeShowLine(res))(i)?,
            103 => map(take(14usize), |res| TempEntityEntity::TeBlood(res))(i)?,
            104 => map(take(9usize), |res| TempEntityEntity::TeDecal(res))(i)?,
            105 => map(take(5usize), |res| TempEntityEntity::TeFizz(res))(i)?,
            106 => map(take(17usize), |res| TempEntityEntity::TeModel(res))(i)?,
            107 => map(take(13usize), |res| TempEntityEntity::TeExplodeModel(res))(i)?,
            // Docs say 13 but its parser says 24.
            108 => map(take(24usize), |res| TempEntityEntity::TeBreakModel(res))(i)?,
            109 => map(take(9usize), |res| TempEntityEntity::TeGunshotDecal(res))(i)?,
            110 => map(take(17usize), |res| TempEntityEntity::TeSpriteSpray(res))(i)?,
            111 => map(take(7usize), |res| TempEntityEntity::TeArmorRicochet(res))(i)?,
            112 => map(take(10usize), |res| TempEntityEntity::TePlayerDecal(res))(i)?,
            113 => map(take(10usize), |res| TempEntityEntity::TeBubbles(res))(i)?,
            114 => map(take(19usize), |res| TempEntityEntity::TeBubbleTrail(res))(i)?,
            115 => map(take(12usize), |res| TempEntityEntity::TeBloodSprite(res))(i)?,
            116 => map(take(7usize), |res| TempEntityEntity::TeWorldDecal(res))(i)?,
            117 => map(take(7usize), |res| TempEntityEntity::TeWorldDecalHigh(res))(i)?,
            118 => map(take(9usize), |res| TempEntityEntity::TeDecalHigh(res))(i)?,
            119 => map(take(16usize), |res| TempEntityEntity::TeProjectile(res))(i)?,
            120 => map(take(18usize), |res| TempEntityEntity::TeSpray(res))(i)?,
            121 => map(take(5usize), |res| TempEntityEntity::TePlayerSprites(res))(i)?,
            122 => map(take(10usize), |res| TempEntityEntity::TeParticleBurst(res))(i)?,
            123 => map(take(9usize), |res| TempEntityEntity::TeFireField(res))(i)?,
            124 => map(take(7usize), |res| {
                TempEntityEntity::TePlayerAttachment(res)
            })(i)?,
            125 => map(take(1usize), |res| {
                TempEntityEntity::TeKillPlayerAttachment(res)
            })(i)?,
            // Docs say 10 but its parser says 18.
            126 => map(take(18usize), |res| TempEntityEntity::TeMultigunShot(res))(i)?,
            127 => map(take(15usize), |res| TempEntityEntity::TeUserTracer(res))(i)?,
            _ => panic!("Bad entity ({})", entity_type),
        };

        Ok((
            i,
            SvcTempEntity {
                entity_type,
                entity,
            },
        ))
    }

    fn write(i: SvcTempEntity) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcTempEntity as u8);

        writer.append_u8(i.entity_type);

        match i.entity {
            TempEntityEntity::TeBeamPoints(i) => {
                writer.append_i16_slice(i.start_position.as_slice());
                writer.append_i16_slice(i.end_position.as_slice());
                writer.append_i16(i.sprite_index);
                writer.append_u8(i.start_frame);
                writer.append_u8(i.frame_rate);
                writer.append_u8(i.life);
                writer.append_u8(i.width);
                writer.append_u8(i.noise);
                writer.append_u8_slice(i.color);
                writer.append_u8(i.speed);
            }
            TempEntityEntity::TeBeamEntPoint(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeGunshot(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeExplosion(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeTarExplosion(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeSmoke(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeTracer(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeLightning(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamEnts(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeSparks(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeLavaSplash(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeTeleport(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeExplosion2(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBspDecal(i) => {
                writer.append_u8_slice(i.unknown1);
                writer.append_i16(i.entity_index);
                if i.entity_index != 0 {
                    writer.append_u8_slice(i.unknown2.unwrap());
                }
            }
            TempEntityEntity::TeImplosion(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeSpriteTrail(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeSprite(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamSprite(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamTorus(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamDisk(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamCylinder(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamFollow(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeGlowSprite(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBeamRing(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeStreakSplash(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeDLight(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeELight(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeTextMessage(i) => {
                writer.append_i8(i.channel);
                writer.append_i16(i.x);
                writer.append_i16(i.y);
                writer.append_i8(i.effect);
                writer.append_u8_slice(i.text_color);
                writer.append_u8_slice(i.effect_color);
                writer.append_i16(i.fade_in_time);
                writer.append_i16(i.fade_out_time);
                writer.append_i16(i.hold_time);

                if i.effect != 0 {
                    writer.append_i16(i.effect_time.unwrap());
                }

                writer.append_u8_slice(i.message);
            }
            TempEntityEntity::TeLine(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBox(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeKillBeam(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeLargeFunnel(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBloodStream(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeShowLine(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBlood(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeDecal(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeFizz(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeModel(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeExplodeModel(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBreakModel(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeGunshotDecal(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeSpriteSpray(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeArmorRicochet(i) => writer.append_u8_slice(i),
            TempEntityEntity::TePlayerDecal(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBubbles(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBubbleTrail(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeBloodSprite(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeWorldDecal(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeWorldDecalHigh(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeDecalHigh(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeProjectile(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeSpray(i) => writer.append_u8_slice(i),
            TempEntityEntity::TePlayerSprites(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeParticleBurst(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeFireField(i) => writer.append_u8_slice(i),
            TempEntityEntity::TePlayerAttachment(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeKillPlayerAttachment(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeMultigunShot(i) => writer.append_u8_slice(i),
            TempEntityEntity::TeUserTracer(i) => writer.append_u8_slice(i),
        }

        writer.data
    }
}
