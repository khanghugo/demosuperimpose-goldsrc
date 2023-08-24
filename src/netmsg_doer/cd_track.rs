use super::*;

pub struct CdTrack {}
impl<'a> NetMsgDoer<'a, SvcCdTrack> for CdTrack {
    fn parse(i: &'a [u8]) -> IResult<&'a [u8], SvcCdTrack> {
        map(tuple((le_i8, le_i8)), |(track, loop_track)| SvcCdTrack {
            track,
            loop_track,
        })(i)
    }

    fn write(i: SvcCdTrack) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcCdTrack as u8);

        writer.append_i8(i.track);
        writer.append_i8(i.loop_track);

        writer.data
    }
}
