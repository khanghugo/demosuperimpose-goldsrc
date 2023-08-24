use super::{utils::BitSliceCast, *};

pub struct PacketEntities {}
impl<'a> NetMsgDoerWithDelta<'a, SvcPacketEntities> for PacketEntities {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcPacketEntities> {
        let clone = i;
        let mut br = BitReader::new(i);

        let entity_count = br.read_n_bit(16).to_owned();
        let mut entity_index = 0;
        let mut entity_states: Vec<EntityState> = vec![];

        loop {
            let footer = br.peek_n_bits(16).to_u16();
            if footer == 0 {
                br.read_n_bit(16);
                break;
            }

            let increment_entity_number = br.read_1_bit();
            let is_absolute_entity_index = if increment_entity_number {
                entity_index += 1;
                None
            } else {
                Some(br.read_1_bit())
            };
            let absolute_entity_index = if is_absolute_entity_index.is_some()
                && is_absolute_entity_index.unwrap()
                && !increment_entity_number
            {
                let val = br.read_n_bit(11).to_owned();
                entity_index = val.to_u16();
                Some(val)
            } else {
                None
            };
            let entity_index_difference = if (is_absolute_entity_index.is_none()
                || (is_absolute_entity_index.is_some() && !is_absolute_entity_index.unwrap()))
                && !increment_entity_number
            {
                let val = br.read_n_bit(6).to_owned();
                entity_index += val.to_u16();
                Some(val)
            } else {
                None
            };
            let has_custom_delta = br.read_1_bit();
            let has_baseline_index = br.read_1_bit();
            let baseline_index = if has_baseline_index {
                Some(br.read_n_bit(6).to_owned())
            } else {
                None
            };
            let between = entity_index > 0 && entity_index <= 32;

            let delta = if between {
                parse_delta(
                    delta_decoders.get("entity_state_player_t\0").unwrap(),
                    &mut br,
                )
            } else {
                if has_custom_delta {
                    parse_delta(
                        delta_decoders.get("custom_entity_state_t\0").unwrap(),
                        &mut br,
                    )
                } else {
                    parse_delta(delta_decoders.get("entity_state_t\0").unwrap(), &mut br)
                }
            };

            entity_states.push(EntityState {
                entity_index,
                increment_entity_number,
                is_absolute_entity_index,
                absolute_entity_index,
                entity_index_difference,
                has_custom_delta,
                has_baseline_index,
                baseline_index,
                delta,
            })
        }

        let range = br.get_consumed_bytes();
        let clone = clone[..range].to_owned();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            SvcPacketEntities {
                entity_count,
                entity_states,
                clone,
            },
        ))
    }

    fn write(i: SvcPacketEntities, delta_decoders: &DeltaDecoderTable) -> Vec<u8> {
        // TODO
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcPacketEntities as u8);

        writer.append_u8_slice(&i.clone);

        writer.data
    }
}
