use super::*;

pub struct UpdateUserInfo {}
impl<'a> NetMsgDoer<'a, SvcUpdateUserInfo<'a>> for UpdateUserInfo {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcUpdateUserInfo<'a>> {
        map(
            tuple((le_u8, le_u32, null_string, take(16usize))),
            |(index, id, user_info, cd_key_hash)| SvcUpdateUserInfo {
                index,
                id,
                user_info,
                cd_key_hash,
            },
        )(i)
    }

    fn write(i: SvcUpdateUserInfo) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcUpdateUserInfo as u8);

        writer.append_u8(i.index);
        writer.append_u32(i.id);
        writer.append_u8_slice(i.user_info);
        writer.append_u8_slice(i.cd_key_hash);

        writer.data
    }
}
