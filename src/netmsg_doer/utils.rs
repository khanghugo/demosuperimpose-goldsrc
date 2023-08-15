use std::collections::HashMap;
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
    // map(count(take_1_bit, n), |what| {
    //     // TODO: make it not this trash
    //     let mut res = BitVec::new();
    //     what.iter().for_each(|e| res.push(*e));
    //     res
    // })

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

pub fn bitslice_to_u8(i: &BitSlice<u8>) -> u8 {
    i.load::<u8>()
}

pub fn bitslice_to_u32(i: &BitSlice<u8>) -> u32 {
    i.load::<u32>()
}

pub fn bitslice_to_i32(i: &BitSlice<u8>) -> i32 {
    i.load::<i32>()
}

pub fn bitslice_to_u8_vec(i: &BitSlice<u8>) -> Vec<u8> {
    i.chunks(8).map(|chunk| bitslice_to_u8(chunk)).collect()
}

fn check_flag(lhs: DeltaType, rhs: DeltaType) -> bool {
    lhs & rhs != 0
}

// Wraps bytes into bits because doing this with nom is a very bad idea.
pub struct BitReader {
    bytes: BitVec<u8, Lsb0>,
    // Bit offset, starting from starting of `bytes`.
    offset: usize,
}

impl BitReader {
    fn new(bytes: &[u8]) -> Self {
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

        while self.peek_byte() != 0 {
            self.offset += 1;
        }

        &self.bytes[start..self.offset]
    }

    /// Peeks 8 bits and converts to utf-8.
    fn peek_byte(&self) -> u8 {
        bitslice_to_u8(&self.bytes[self.offset..self.offset + 8])
    }

    fn get_offset(&self) -> usize {
        self.offset
    }
}
// Parser input takes in bit stream. Any struct afterward has to be included with delta parsing
// because padding is done at the end of the message, not delta.
// Bit parsing will be done imperatively with custom bit parsing and consumed accordingly for every byte with nom
pub fn parse_delta(dd: DeltaDecoder, br: &mut BitReader) -> HashMap<String, Vec<u8>> {
    let mut res: HashMap<String, Vec<u8>> = HashMap::new();

    let mask_byte_count = bitslice_to_u8(br.read_n_bit(3)) as usize;
    let mut mask_byte = vec![0u8; mask_byte_count];

    for i in 0..mask_byte_count {
        for j in 0..8 {
            let index = j + i * 8;

            if index == dd.len() {
                return res;
            }

            if (mask_byte[i] & (1 << j)) == 0 {
                return res;
            }

            let curr = &dd[index];
            let lhs = curr.flags;
            let key = from_utf8(curr.name).unwrap().to_owned();
            if check_flag(lhs, DeltaType::Angle) {
                let value = bitslice_to_i32(br.read_n_bit(curr.bits as usize));
                let multiplier = 360f32 / (1 << curr.bits) as f32;

                let res_value = (value as f32 * multiplier).to_le_bytes();

                res.insert(key, res_value.to_vec());
            } else if check_flag(lhs, DeltaType::String) {
                res.insert(key, bitslice_to_u8_vec(br.read_string()));
            } else {
                // TODO minor refactor because oCD
                // TODO read signed can just read normal 32 then cast accordingly
                let (sign, value) = if check_flag(lhs, DeltaType::Signed) {
                    let sign = br.read_1_bit();
                    let value = bitslice_to_i32(br.read_n_bit(curr.bits as usize - 1));
                    if sign {
                        (-1, value)
                    } else {
                        (1, value)
                    }
                } else {
                    (1, bitslice_to_i32(br.read_n_bit(curr.bits as usize)))
                };
                let divisor = curr.divisor;

                let res_value = (sign * value) as f32 / divisor as f32;
                let res_value = res_value.to_le_bytes();

                res.insert(key, res_value.to_vec());
            }
        }
    }

    res
}
