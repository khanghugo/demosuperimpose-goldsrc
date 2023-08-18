use super::{
    utils::{get_initial_delta, parse_delta, BitReader},
    *,
};

pub struct ClientData {}
impl<'a> NetMsgDoer<'a, SvcClientData> for ClientData {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
    ) -> IResult<&'a [u8], SvcClientData> {
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

        // Remember to write the last "false" bit.

        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((
            i,
            SvcClientData {
                has_delta_update_mask,
                delta_update_mask,
                client_data,
                weapon_data: Some(weapon_data),
            },
        ))
    }

    fn write(i: SvcClientData) -> Vec<u8> {
        todo!()
    }
}
