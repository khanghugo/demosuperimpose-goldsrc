use super::{utils::BitSliceCast, *};

pub struct SpawnBaseline {}
impl<'a> NetMsgDoer<'a, SvcSpawnBaseline> for SpawnBaseline {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcSpawnBaseline> {
        let clone = i;
        let mut br = BitReader::new(i);
        let mut entities: Vec<EntityS> = vec![];

        loop {
            let index = br.read_n_bit(11).to_owned();

            if index.to_u16() == ((1 << 11) - 1) {
                break;
            }

            let between = index.to_u16() > 0 && index.to_u16() <= 32;

            let type_ = br.read_n_bit(2).to_owned();

            let delta = if type_.to_u8() & 1 != 0 {
                if between {
                    parse_delta(
                        delta_decoders.get("entity_state_player_t\0").unwrap(),
                        &mut br,
                    )
                } else {
                    parse_delta(delta_decoders.get("entity_state_t\0").unwrap(), &mut br)
                }
            } else {
                parse_delta(
                    delta_decoders.get("custom_entity_state_t\0").unwrap(),
                    &mut br,
                )
            };

            let res = EntityS {
                index,
                type_,
                delta,
            };

            entities.push(res);

            // let index = br.peek_n_bits(11);
            // if index.to_u16() == ((1 << 11) - 1) {
            //     br.read_n_bit(11);
            //     break;
            // }
        }

        let footer = br.read_n_bit(5).to_owned();
        if footer.to_u8() != (1 << 5) - 1 {
            panic!("Bad spawn baseline");
        }

        let total_extra_data = br.read_n_bit(6).to_owned();

        let extra_data: Vec<Delta> = (0..total_extra_data.to_u8())
            .map(|_| parse_delta(delta_decoders.get("entity_state_t\0").unwrap(), &mut br))
            .collect();

        let range = br.get_consumed_bytes();
        let clone = clone[..range].to_owned();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            SvcSpawnBaseline {
                entities,
                footer,
                total_extra_data,
                extra_data,
                clone,
            },
        ))
    }

    fn write(i: SvcSpawnBaseline) -> Vec<u8> {
        // TODO
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcSpawnBaseline as u8);

        writer.append_u8_slice(&i.clone);

        writer.data
    }
}
