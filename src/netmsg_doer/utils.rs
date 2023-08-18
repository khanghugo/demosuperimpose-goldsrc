use std::str::from_utf8;

use bitvec::prelude::*;
use bitvec::vec::BitVec;

use super::*;

// Eh, I am not sure how to do inclusive until so here this is instead.
pub fn null_string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, string) = peek(terminated(take_until("\x00"), tag("\x00")))(i)?;
    take(string.len() + 1)(i)
}

pub fn take_n_bit<'a>(
    n: usize,
) -> impl FnMut((&'a [u8], usize)) -> IResult<(&[u8], usize), Vec<bool>> {
    count(take_1_bit, n)
}

pub fn take_1_bit(i: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    map(take_bit(1usize), |what: u8| what != 0)(i)
}

pub fn vec_bool_to_u8(i: Vec<bool>) -> u8 {
    if i.len() > 8 {
        panic!("Length {} is greater than 8.", i.len());
    }

    i.iter()
        .enumerate()
        .fold(0u8, |acc, (idx, e)| acc | ((*e as u8) << idx))
}

pub fn vec_bool_to_u32(i: Vec<bool>) -> u32 {
    if i.len() > 32 {
        panic!("Length {} is greater than 32.", i.len());
    }

    i.iter()
        .enumerate()
        .fold(0u32, |acc, (idx, e)| acc | ((*e as u32) << idx))
}

pub trait BitSliceCast {
    fn to_u8(&self) -> u8;
    fn to_i8(&self) -> i8;
    fn to_u16(&self) -> u16;
    fn to_i16(&self) -> i16;
    fn to_u32(&self) -> u32;
    fn to_i32(&self) -> i32;
}

impl BitSliceCast for BitSlice<u8> {
    // https://github.com/ferrilab/bitvec/issues/64
    fn to_u8(&self) -> u8 {
        self.load::<u8>()
    }

    fn to_i8(&self) -> i8 {
        self.load::<i8>()
    }

    fn to_u16(&self) -> u16 {
        self.load::<u16>()
    }

    fn to_i16(&self) -> i16 {
        self.load::<i16>()
    }

    fn to_u32(&self) -> u32 {
        self.load::<u32>()
    }

    fn to_i32(&self) -> i32 {
        self.load::<i32>()
    }
}

pub fn bitslice_to_u8_vec(i: &BitSlice<u8>) -> Vec<u8> {
    i.chunks(8).map(|chunk| chunk.to_u8()).collect()
}

fn check_flag(lhs: u32, rhs: DeltaType) -> bool {
    lhs as u32 & rhs as u32 != 0
}

// Wraps bytes into bits because doing this with nom is a very bad idea.
pub struct BitReader {
    pub bytes: BitVec<u8, Lsb0>,
    // Bit offset, starting from starting of `bytes`.
    offset: usize,
}

impl BitReader {
    pub fn new(bytes: &[u8]) -> Self {
        BitReader {
            bytes: BitVec::from_slice(bytes),
            offset: 0,
        }
    }

    pub fn read_1_bit(&mut self) -> bool {
        let res = self.bytes[self.offset];
        self.offset += 1;
        res
    }

    pub fn read_n_bit(&mut self, n: usize) -> &BitSlice<u8> {
        let range = self.offset + n;
        let res: &BitSlice<u8, Lsb0> = &self.bytes[self.offset..range];
        self.offset += n;
        res
    }

    pub fn read_string(&mut self) -> &BitSlice<u8> {
        let start = self.offset;

        // The second condition is to make sure we are aligned.
        while self.peek_byte() != 0 || (self.peek_byte() == 0 && (self.offset - start) % 8 != 0) {
            self.offset += 1;
        }

        // Includes the null terminator.
        self.offset += 8;

        &self.bytes[start..self.offset]
    }

    /// Peeks 8 bits and converts to u8.
    fn peek_byte(&self) -> u8 {
        self.peek_n_bits(8).to_u8()
    }

    pub fn peek_n_bits(&self, n: usize) -> &BitSlice<u8> {
        &self.bytes[self.offset..self.offset + n]
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of bits read into bytes.
    pub fn get_consumed_bytes(&self) -> usize {
        let current_bit = self.get_offset();
        let modulo = current_bit % 8;
        let remaining_bits = if modulo == 0 { 0 } else { 8 - modulo };
        let consumed_bytes = (current_bit + remaining_bits) / 8;

        consumed_bytes
    }
}

// very big TODO: do whole thing with nom
// chaining conditions right now with nom is a bit very difficult to work with
/// https://github.com/skyrim/hlviewer.js/blob/master/src/Replay/readDelta.ts
///
/// To parse delta, we first have to construct the delta decoder table.
///
/// Info regarding delta decoder will arrive on the first frame of the first demo directory.
///
/// The netmessage heavily occupy that frame will be delta description.
///
/// After parsing the message, we will have our delta decoder for subsequent delta parsing.
///
pub fn parse_delta<'a>(dd: &DeltaDecoder, br: &'a mut BitReader) -> Delta {
    let mut res: Delta = Delta::new();
    let mask_byte_count = br.read_n_bit(3).to_u8() as usize;
    let mask_byte: Vec<u8> = (0..mask_byte_count)
        .map(|_| br.read_n_bit(8).to_u8())
        .collect();

    for i in 0..mask_byte_count {
        for j in 0..8 {
            let index = j + i * 8;

            if index == dd.len() {
                return res;
            }

            if (mask_byte[i] & (1 << j)) != 0 {
                let curr = &dd[index];
                let lhs = curr.flags;
                let key = from_utf8(&curr.name).unwrap().to_owned();

                if check_flag(lhs, DeltaType::Byte) {
                    if check_flag(lhs, DeltaType::Signed) {
                        let sign = if br.read_1_bit() { -1 } else { 1 };
                        let value = br.read_n_bit(curr.bits as usize - 1).to_i8();
                        let res_value = ((sign * value) / curr.divisor as i8).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    } else {
                        let value = (br.read_n_bit(curr.bits as usize)).to_u8();
                        let res_value = (value / curr.divisor as u8).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    }
                } else if check_flag(lhs, DeltaType::Short) {
                    if check_flag(lhs, DeltaType::Signed) {
                        let sign = if br.read_1_bit() { -1 } else { 1 };
                        let value = (br.read_n_bit(curr.bits as usize - 1)).to_i16();
                        let res_value = ((sign * value) / curr.divisor as i16).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    } else {
                        let value = (br.read_n_bit(curr.bits as usize)).to_u16();
                        let res_value = (value / curr.divisor as u16).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    }
                } else if check_flag(lhs, DeltaType::Integer) {
                    if check_flag(lhs, DeltaType::Signed) {
                        let sign = if br.read_1_bit() { -1 } else { 1 };
                        let value = (br.read_n_bit(curr.bits as usize - 1)).to_i32();
                        let res_value = ((sign * value) / curr.divisor as i32).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    } else {
                        let value = (br.read_n_bit(curr.bits as usize)).to_u32();
                        let res_value = (value / curr.divisor as u32).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    }
                } else if check_flag(lhs, DeltaType::Float)
                    || check_flag(lhs, DeltaType::TimeWindow8)
                    || check_flag(lhs, DeltaType::TimeWindowBig)
                {
                    if check_flag(lhs, DeltaType::Signed) {
                        let sign = if br.read_1_bit() { -1 } else { 1 };
                        let value = (br.read_n_bit(curr.bits as usize - 1)).to_i32();
                        let res_value =
                            (((sign * value) as f32) / (curr.divisor as f32)).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    } else {
                        let value = (br.read_n_bit(curr.bits as usize)).to_u32();
                        let res_value = ((value as f32) / (curr.divisor as f32)).to_le_bytes();
                        res.insert(key, res_value.to_vec());
                    }
                } else if check_flag(lhs, DeltaType::Angle) {
                    let value = (br.read_n_bit(curr.bits as usize)).to_i32();
                    let multiplier = 360f32 / (1 << curr.bits) as f32;
                    let res_value = (value as f32 * multiplier).to_le_bytes();
                    res.insert(key, res_value.to_vec());
                } else if check_flag(lhs, DeltaType::String) {
                    res.insert(key, bitslice_to_u8_vec(br.read_string()));
                } else {
                    panic!("Decoded value does not match any types. Should this happens?");
                }
            }
        }
    }

    res
}

pub fn get_initial_delta() -> DeltaDecoderTable {
    let mut res: DeltaDecoderTable = DeltaDecoderTable::new();

    let e1 = DeltaDecoderS {
        name: "flags".into(),
        bits: 32,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e2 = DeltaDecoderS {
        name: "name".into(),
        bits: 8,
        divisor: 1.,
        flags: DeltaType::String as u32,
    };
    let e3 = DeltaDecoderS {
        name: "offset".into(),
        bits: 16,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e4 = DeltaDecoderS {
        name: "size".into(),
        bits: 8,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e5 = DeltaDecoderS {
        name: "bits".into(),
        bits: 8,
        divisor: 1.,
        flags: DeltaType::Integer as u32,
    };
    let e6 = DeltaDecoderS {
        name: "divisor".into(),
        bits: 32,
        divisor: 4000.,
        flags: DeltaType::Float as u32,
    };
    let e7 = DeltaDecoderS {
        name: "preMultiplier".into(),
        bits: 32,
        divisor: 4000.,
        flags: DeltaType::Float as u32,
    };

    let default_decoder = vec![e1, e2, e3, e4, e5, e6, e7];

    res.insert("delta_description_t\0".to_string(), default_decoder);

    res
}
