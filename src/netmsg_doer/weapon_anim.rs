use super::*;

pub struct WeaponAnim {}
impl<'a> NetMsgDoer<'a, SvcWeaponAnim> for WeaponAnim {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcWeaponAnim> {
        map(
            tuple((le_i8, le_i8)),
            |(sequence_number, weapon_model_body_group)| SvcWeaponAnim {
                sequence_number,
                weapon_model_body_group,
            },
        )(i)
    }

    fn write(i: SvcWeaponAnim) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcWeaponAnim as u8);

        writer.append_i8(i.sequence_number);
        writer.append_i8(i.weapon_model_body_group);

        writer.data
    }
}
