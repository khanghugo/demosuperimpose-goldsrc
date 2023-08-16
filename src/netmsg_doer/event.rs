use super::{
    utils::{parse_delta, take_n_bit, BitReader, BitSliceCast},
    *,
};

pub struct Event;
// impl<'a> NetMsgDoer<'a, SvcEvent<'_>> for Event {
//     fn parse(i: &'a [u8]) -> IResult<&[u8], SvcEvent> {
//         let br = BitReader::new(i);
//         let event_count = br.read_n_bit(5).to_u8();
//         let events: Vec<EventS> = (0..event_count)
//             .map(|_| {
//                 let event_index = br.read_n_bit(10);
//                 let has_packet_index = br.read_1_bit();
//                 let packet_index = if has_packet_index {
//                     Some(br.read_n_bit(11))
//                 } else {
//                     None
//                 };
//                 let has_delta = if has_packet_index {
//                     Some(br.read_1_bit())
//                 } else {
//                     None
//                 };
//                 let delta = if has_delta.is_some() {
//                     Some(parse_delta(dd, &mut br))
//                 } else {
//                     None
//                 };
//             })
//             .collect();
//     }

//     fn write(i: SvcEvent) -> Vec<u8> {
//         // let mut writer: ByteWriter::new();
//     }
// }
