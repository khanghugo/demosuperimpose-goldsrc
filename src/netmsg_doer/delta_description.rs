use super::*;
use std::convert::TryInto;

pub struct DeltaDescription {}
impl<'a> NetMsgDoerWithDelta<'a, SvcDeltaDescription<'a>> for DeltaDescription {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcDeltaDescription<'a>> {
        let (i, name) = null_string(i)?;
        let (i, total_fields) = le_u16(i)?;

        let clone = i;

        // Delta description is usually in LOADING section and first frame message.
        // It will detail the deltas being used and its index for correct decoding.
        // So this would be the only message that modifies the delta decode table.

        let mut br = BitReader::new(i);
        let data: Vec<Delta> = (0..total_fields)
            .map(|_| {
                parse_delta(
                    delta_decoders.get("delta_description_t\0").unwrap(),
                    &mut br,
                )
            })
            .collect();

        let decoder: DeltaDecoder = data
            .iter()
            .map(|entry| {
                DeltaDecoderS {
                    name: entry.get("name").unwrap().to_owned(),
                    bits: u32::from_le_bytes(
                        entry.get("bits").unwrap().as_slice().try_into().unwrap(),
                    ), // heh
                    divisor: f32::from_le_bytes(
                        entry.get("divisor").unwrap().as_slice().try_into().unwrap(),
                    ),
                    flags: u32::from_le_bytes(
                        entry.get("flags").unwrap().as_slice().try_into().unwrap(),
                    ),
                    should_write: false,
                }
            })
            .collect();

        let range = br.get_consumed_bytes();
        let clone = clone[..range].to_owned();
        let (i, _) = take(range)(i)?;

        // It really should mutate the delta decoder table here but we're respecting ownership.
        Ok((
            i,
            SvcDeltaDescription {
                name,
                total_fields,
                fields: decoder,
                clone,
            },
        ))
    }

    fn write(i: SvcDeltaDescription<'a>, delta_decoders: &DeltaDecoderTable) -> Vec<u8> {
        // TODO
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcDeltaDescription as u8);

        writer.append_u8_slice(i.name);
        writer.append_u16(i.total_fields);
        writer.append_u8_slice(&i.clone);

        writer.data
    }
}
