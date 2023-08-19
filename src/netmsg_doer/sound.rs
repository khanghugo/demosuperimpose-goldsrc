use bitvec::vec::BitVec;

use super::{utils::BitSliceCast, *};
use bitvec::prelude::Lsb0;

pub struct Sound {}
impl<'a> NetMsgDoer<'a, SvcSound> for Sound {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcSound> {
        let mut br = BitReader::new(i);

        let flags = br.read_n_bit(9).to_owned();
        let flag_u = flags.to_u32();
        let volume = if flag_u & 1 != 0 {
            Some(br.read_n_bit(8).to_owned())
        } else {
            None
        };
        let attenuation = if flag_u & 2 != 0 {
            Some(br.read_n_bit(8).to_owned())
        } else {
            None
        };
        let channel = br.read_n_bit(3).to_owned();
        let entity_index = br.read_n_bit(11).to_owned();
        let (sound_index_long, sound_index_short) = if flag_u & 4 != 0 {
            (Some(br.read_n_bit(16).to_owned()), None)
        } else {
            (None, Some(br.read_n_bit(8).to_owned()))
        };
        let (has_x, has_y, has_z) = (br.read_1_bit(), br.read_1_bit(), br.read_1_bit());
        let origin_x = if has_x {
            Some(parse_origin(&mut br))
        } else {
            None
        };
        let origin_y = if has_y {
            Some(parse_origin(&mut br))
        } else {
            None
        };
        let origin_z = if has_z {
            Some(parse_origin(&mut br))
        } else {
            None
        };
        let pitch = if flag_u & 8 != 0 {
            br.read_n_bit(8).to_owned()
        } else {
            BitVec::<u8, Lsb0>::from_element(1u8)
        };

        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((
            i,
            SvcSound {
                flags,
                volume,
                attenuation,
                channel,
                entity_index,
                sound_index_long,
                sound_index_short,
                has_x,
                has_y,
                has_z,
                origin_x,
                origin_y,
                origin_z,
                pitch,
            },
        ))
    }

    fn write(i: SvcSound) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSound as u8);

        let mut bw = BitWriter::new();

        let should_write_volume = i.flags.to_u16() & 1 != 0;
        let should_write_attenuation = i.flags.to_u16() & 2 != 0;
        let should_write_sound_index_long = i.flags.to_u16() & 4 != 0;
        let should_write_pitch = i.flags.to_u16() & 8 != 0;

        bw.append_vec(i.flags);

        if should_write_volume {
            bw.append_vec(i.volume.unwrap());
        }

        if should_write_attenuation {
            bw.append_vec(i.attenuation.unwrap());
        }

        bw.append_vec(i.channel);
        bw.append_vec(i.entity_index);

        if should_write_sound_index_long {
            bw.append_vec(i.sound_index_long.unwrap())
        } else {
            bw.append_vec(i.sound_index_short.unwrap())
        }

        bw.append_bit(i.has_x);
        bw.append_bit(i.has_y);
        bw.append_bit(i.has_z);

        if i.has_x {
            write_origin(i.origin_x.unwrap(), &mut bw);
        }
        if i.has_y {
            write_origin(i.origin_y.unwrap(), &mut bw);
        }
        if i.has_z {
            write_origin(i.origin_z.unwrap(), &mut bw);
        }

        if should_write_pitch {
            bw.append_vec(i.pitch);
        }

        writer.append_u8_slice(bw.get_u8_vec().as_slice());

        writer.data
    }
}

fn parse_origin(br: &mut BitReader) -> OriginCoord {
    let int_flag = br.read_1_bit();
    let fraction_flag = br.read_1_bit();

    let is_negative = if int_flag || fraction_flag {
        Some(br.read_1_bit())
    } else {
        None
    };

    let int_value = if int_flag {
        Some(br.read_n_bit(12).to_owned())
    } else {
        None
    };

    let fraction_value = if fraction_flag {
        Some(br.read_n_bit(3).to_owned())
    } else {
        None
    };

    // let unknown = br.read_n_bit(2).to_owned();

    OriginCoord {
        int_flag,
        fraction_flag,
        is_negative,
        int_value,
        fraction_value,
        // unknown,
    }
}

fn write_origin(i: OriginCoord, bw: &mut BitWriter) {
    bw.append_bit(i.int_flag);
    bw.append_bit(i.fraction_flag);

    if let Some(negative) = i.is_negative {
        bw.append_bit(negative);
    }

    if let Some(int) = i.int_value {
        bw.append_vec(int);
    }

    if let Some(frac) = i.fraction_value {
        bw.append_vec(frac);
    }

    // bw.append_vec(i.unknown);
}

// This is how to interpret the number.
// if !int_flag && !fraction_flag {
//     return 0.;
// }
// let is_negative = br.read_1_bit();
// let int_value = if int_flag {
//     br.read_n_bit(12).to_u16()
// } else {
//     0
// };
// let fraction_value = if fraction_flag {
//     br.read_n_bit(3).to_u16()
// } else {
//     0
// };
// let value = f32::from(int_value) + f32::from(fraction_value) / 32.;
// if is_negative {
//     value * -1.
// } else {
//     value
// }
