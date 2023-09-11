use super::{utils::write_delta, *};

pub struct EventReliable {}
impl<'a> NetMsgDoerWithDelta<'a, SvcEventReliable> for EventReliable {
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

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

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

    fn write(i: SvcEventReliable, delta_decoders: &mut DeltaDecoderTable) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(EngineMessageType::SvcEventReliable as u8);

        bw.append_vec(i.event_index);
        write_delta(
            &i.event_args,
            delta_decoders.get_mut("event_t\0").unwrap(),
            &mut bw,
        );

        bw.append_bit(i.has_fire_time);
        if i.has_fire_time {
            bw.append_vec(i.fire_time.unwrap());
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
