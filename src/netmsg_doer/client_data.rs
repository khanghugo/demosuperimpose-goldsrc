use super::{
    utils::{parse_delta, write_delta, BitReader},
    *,
};

pub struct ClientData {}
impl<'a> NetMsgDoerWithDelta<'a, SvcClientData> for ClientData {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcClientData> {
        let clone = i;
        let mut br = BitReader::new(i);

        let has_delta_update_mask = br.read_1_bit();
        let delta_update_mask = if has_delta_update_mask {
            Some(br.read_n_bit(8).to_owned())
        } else {
            None
        };

        let client_data = parse_delta(delta_decoders.get("clientdata_t\0").unwrap(), &mut br);

        // This is a vector unlike THE docs.
        let mut weapon_data: Vec<ClientDataWeaponData> = vec![];
        while br.read_1_bit() {
            let weapon_index = br.read_n_bit(6).to_owned();
            let delta = parse_delta(delta_decoders.get("weapon_data_t\0").unwrap(), &mut br);

            weapon_data.push(ClientDataWeaponData {
                weapon_index,
                weapon_data: delta,
            });
        }
        
        let weapon_data = if weapon_data.is_empty() {
            None
        } else {
            Some(weapon_data)
        };

        // Remember to write the last "false" bit.

        let range = br.get_consumed_bytes();
        let clone = clone[..range].to_owned();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            SvcClientData {
                has_delta_update_mask,
                delta_update_mask,
                client_data,
                weapon_data,
                clone,
            },
        ))
    }

    fn write(i: SvcClientData, delta_decoders: &DeltaDecoderTable) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(EngineMessageType::SvcClientData as u8);

        // bw.append_bit(i.has_delta_update_mask);

        // if i.has_delta_update_mask {
        //     bw.append_vec(i.delta_update_mask.unwrap());
        // }

        // write_delta(
        //     &i.client_data,
        //     delta_decoders.get("clientdata_t\0").unwrap(),
        //     &mut bw,
        // );

        // if let Some(weapon_data) = i.weapon_data {
        //     for data in weapon_data {
        //         bw.append_bit(true);
        //         bw.append_vec(data.weapon_index);
        //         write_delta(
        //             &data.weapon_data,
        //             delta_decoders.get("weapon_data_t\0").unwrap(),
        //             &mut bw,
        //         );
        //     }
        // }

        // // false bit for weapon data
        // bw.append_bit(false);

        // writer.append_u8_slice(&bw.get_u8_vec());

        writer.append_u8_slice(&i.clone);

        writer.data
    }
}
