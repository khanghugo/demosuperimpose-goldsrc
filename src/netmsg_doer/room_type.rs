use super::*;

pub struct RoomType {}
impl<'a> NetMsgDoer<'a, SvcRoomType> for RoomType {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcRoomType> {
        map(le_u16, |room_type| SvcRoomType { room_type })(i)
    }

    fn write(i: SvcRoomType) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcRoomType as u8);

        writer.append_u16(i.room_type);

        writer.data
    }
}
