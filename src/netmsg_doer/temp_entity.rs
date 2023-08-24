use super::*;

pub struct TempEntity {}
impl<'a> NetMsgDoer<'a, SvcTempEntity<'a>> for TempEntity {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcTempEntity<'a>> {
        let (i, entity_type) = le_u8(i)?;

        let (i, entity) = match entity_type {
            0 => map(take(24usize), |res| TempEntityEntity::TeBeamPoints(res))(i)?,
            1 => map(take(20usize), |res| TempEntityEntity::TeBeamEntPoint(res))(i)?,
            2 => map(take(6usize), |res| TempEntityEntity::TeGunshot(res))(i)?,
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
                    map(take(2usize), |what| Some(what))(i)?
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
            15 => map(take(19usize), |res| TempEntityEntity::TeSpriteRail(res))(i)?,
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
                    map(le_i16, |what| Some(what))(i)?
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
            TempEntityEntity::TeBeamPoints(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamEntPoint(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeGunshot(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeExplosion(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeTarExplosion(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeSmoke(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeTracer(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeLightning(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamEnts(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeSparks(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeLavaSplash(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeTeleport(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeExplosion2(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBspDecal(what) => {
                writer.append_u8_slice(what.unknown1);
                writer.append_i16(what.entity_index);
                if what.entity_index != 0 {
                    writer.append_u8_slice(what.unknown2.unwrap());
                }
            }
            TempEntityEntity::TeImplosion(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeSpriteRail(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeSprite(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamSprite(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamTorus(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamDisk(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamCylinder(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamFollow(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeGlowSprite(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBeamRing(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeStreakSplash(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeDLight(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeELight(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeTextMessage(what) => {
                writer.append_i8(what.channel);
                writer.append_i16(what.x);
                writer.append_i16(what.y);
                writer.append_i8(what.effect);
                writer.append_u8_slice(what.text_color);
                writer.append_u8_slice(what.effect_color);
                writer.append_i16(what.fade_in_time);
                writer.append_i16(what.fade_out_time);
                writer.append_i16(what.hold_time);

                if what.effect != 0 {
                    writer.append_i16(what.effect_time.unwrap());
                }

                writer.append_u8_slice(what.message);
            }
            TempEntityEntity::TeLine(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBox(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeKillBeam(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeLargeFunnel(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBloodStream(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeShowLine(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBlood(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeDecal(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeFizz(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeModel(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeExplodeModel(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBreakModel(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeGunshotDecal(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeSpriteSpray(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeArmorRicochet(what) => writer.append_u8_slice(what),
            TempEntityEntity::TePlayerDecal(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBubbles(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBubbleTrail(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeBloodSprite(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeWorldDecal(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeWorldDecalHigh(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeDecalHigh(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeProjectile(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeSpray(what) => writer.append_u8_slice(what),
            TempEntityEntity::TePlayerSprites(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeParticleBurst(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeFireField(what) => writer.append_u8_slice(what),
            TempEntityEntity::TePlayerAttachment(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeKillPlayerAttachment(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeMultigunShot(what) => writer.append_u8_slice(what),
            TempEntityEntity::TeUserTracer(what) => writer.append_u8_slice(what),
        }

        writer.data
    }
}
