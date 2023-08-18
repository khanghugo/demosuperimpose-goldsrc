use super::*;

pub struct EventReliable {}
impl<'a> NetMsgDoer<'a, SvcEventReliable> for EventReliable {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcEventReliable> {
        let mut br = BitReader::new(i);

        let event_index = br.read_n_bit(10).to_owned();
        let event_args = parse_delta(delta_decoders.get("event_t\0").unwrap(), &mut br);
        let has_fire_time = br.read_1_bit();
        let fire_time = if has_fire_time {
            Some(br.read_n_bit(16).to_owned())
        } else {
            None
        };

        Ok((
            i,
            SvcEventReliable {
                event_index,
                event_args,
                has_fire_time,
                fire_time,
            },
        ))
    }

    fn write(i: SvcEventReliable) -> Vec<u8> {
        todo!();
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcEventReliable as u8);

        writer.data
    }
}
