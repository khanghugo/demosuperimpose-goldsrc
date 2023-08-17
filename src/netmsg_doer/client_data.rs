use super::{
    utils::{get_initial_delta, parse_delta, BitReader},
    *,
};

pub struct ClientData {}
impl<'a> NetMsgDoer<'a, SvcClientData> for ClientData {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcClientData> {
        let mut br = BitReader::new(i);

        let has_delta_update_mask = br.read_1_bit();
        let delta_update_mask = if has_delta_update_mask {
            Some(br.read_n_bit(8).to_owned())
        } else {
            None
        };
        let initial = get_initial_delta();
        let client_data = parse_delta(initial.get("delta_description_t").unwrap(), &mut br);
        let has_weapon_data = br.read_1_bit();
        let weapon_index = if has_weapon_data {
            Some(br.read_n_bit(6).to_owned())
        } else {
            None
        };
        let weapon_data = if has_weapon_data {
            Some(parse_delta(
                initial.get("delta_description_t").unwrap(),
                &mut br,
            ))
        } else {
            None
        };

        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((
            i,
            SvcClientData {
                has_delta_update_mask,
                delta_update_mask,
                client_data,
                has_weapon_data,
                weapon_index,
                weapon_data,
            },
        ))
    }

    fn write(i: SvcClientData) -> Vec<u8> {
        todo!()
    }
}
