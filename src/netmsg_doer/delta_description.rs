use super::*;

pub struct DeltaDescription {}
impl<'a> NetMsgDoer<'a, SvcDeltaDescription<'a>> for DeltaDescription {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcDeltaDescription<'a>> {
        let (i, name) = null_string(i)?;
        let (i, total_fields) = le_u16(i)?;

        // Delta description is usually in LOADING section and first frame message.
        // It will detail the deltas being used and its index for correct decoding.
        // So this would be the only message that modifies the delta decode table.
        // delta_decoders.insert(from_utf8(name).unwrap().to_string(), vec![]);
        // delta_decoders.get(k)

        let mut br = BitReader::new(i);
        let data: Vec<Delta> = (0..total_fields)
            .map(|_| parse_delta(delta_decoders.get("delta_description_t").unwrap(), &mut br))
            .collect();

        println!("{:?}", data);
        todo!()
    }

    fn write(i: SvcDeltaDescription<'a>) -> Vec<u8> {
        todo!()
    }
}
