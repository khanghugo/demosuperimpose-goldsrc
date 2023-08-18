use super::*;

pub struct ResourceRequest {}
impl<'a> NetMsgDoer<'a, SvcResourceRequest> for ResourceRequest {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcResourceRequest> {
        map(
            tuple((le_i32, count(le_u8, 4usize))),
            |(spawn_count, unknown)| SvcResourceRequest {
                spawn_count,
                unknown,
            },
        )(i)
    }

    fn write(i: SvcResourceRequest) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcResourceRequest as u8);

        writer.append_i32(i.spawn_count);

        for what in i.unknown {
            writer.append_u8(what);
        }

        writer.data
    }
}
