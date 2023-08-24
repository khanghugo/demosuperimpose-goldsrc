use super::*;

pub struct EventReliable {}
impl<'a> NetMsgDoerWithDelta<'a, SvcEventReliable> for EventReliable {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcEventReliable> {
        let clone = i;
        let mut br = BitReader::new(i);

        let event_index = br.read_n_bit(10).to_owned();
        let event_args = parse_delta(delta_decoders.get("event_t\0").unwrap(), &mut br);
        let has_fire_time = br.read_1_bit();
        let fire_time = if has_fire_time {
            Some(br.read_n_bit(16).to_owned())
        } else {
            None
        };

        let range = br.get_consumed_bytes();
        let clone = clone[..range].to_owned();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            SvcEventReliable {
                event_index,
                event_args,
                has_fire_time,
                fire_time,
                clone,
            },
        ))
    }

    fn write(i: SvcEventReliable, delta_decoders: &DeltaDecoderTable) -> Vec<u8> {
        // TODO
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcEventReliable as u8);

        writer.append_u8_slice(&i.clone);

        writer.data
    }
}
