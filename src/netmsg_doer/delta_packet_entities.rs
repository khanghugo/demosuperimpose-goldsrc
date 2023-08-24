use super::{utils::BitSliceCast, *};

pub struct DeltaPacketEntities {}
impl<'a> NetMsgDoerWithDelta<'a, SvcDeltaPacketEntities> for DeltaPacketEntities {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcDeltaPacketEntities> {
        let clone = i;
        let mut br = BitReader::new(i);

        let entity_count = br.read_n_bit(16).to_owned();
        let delta_sequence = br.read_n_bit(8).to_owned();

        let mut entity_index: u16 = 0;
        let mut entity_states: Vec<EntityStateDelta> = vec![];

        loop {
            let footer = br.peek_n_bits(16).to_u16();
            if footer == 0 {
                br.read_n_bit(16);
                break;
            }

            let remove_entity = br.read_1_bit();
            let is_absolute_entity_index = br.read_1_bit();

            let (absolute_entity_index, entity_index_difference) = if is_absolute_entity_index {
                let idx = br.read_n_bit(11).to_owned();
                entity_index = idx.to_u16();
                (Some(idx), None)
            } else {
                let diff = br.read_n_bit(6).to_owned();
                entity_index += diff.to_u16();
                (None, Some(diff))
            };

            if remove_entity {
                continue;
            }

            let has_custom_delta = br.read_1_bit();
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

            entity_states.push(EntityStateDelta {
                entity_index,
                remove_entity,
                is_absolute_entity_index,
                absolute_entity_index,
                entity_index_difference,
                has_custom_delta,
                delta,
            });
        }

        let range = br.get_consumed_bytes();
        let clone = clone[..range].to_owned();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            SvcDeltaPacketEntities {
                entity_count,
                delta_sequence,
                entity_states,
                clone,
            },
        ))
    }

    fn write(i: SvcDeltaPacketEntities, delta_decoders: &DeltaDecoderTable) -> Vec<u8> {
        // TODO
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcDeltaPacketEntities as u8);

        writer.append_u8_slice(&i.clone);

        writer.data
    }
}
