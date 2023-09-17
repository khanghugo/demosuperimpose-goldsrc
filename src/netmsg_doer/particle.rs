use super::*;

pub struct Particle {}
impl<'a> NetMsgDoer<'a, SvcParticle<'a>> for Particle {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcParticle> {
        map(
            tuple((count(le_i16, 3), take(3usize), le_u8, le_u8)),
            |(origin, direction, count, color)| SvcParticle {
                origin,
                direction,
                count,
                color,
            },
        )(i)
    }

    fn write(i: SvcParticle) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcParticle as u8);

        for j in 0..3 {
            writer.append_i16(i.origin[j])
        }
        writer.append_u8_slice(i.direction);
        writer.append_u8(i.count);
        writer.append_u8(i.color);

        writer.data
    }
}
