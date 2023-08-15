use super::{utils::take_n_bit, *};

pub struct Event;
// impl<'a> NetMsgDoer<'a, SvcEvent<'_>> for Event {
//     fn parse(i: &'a [u8]) -> IResult<&[u8], SvcEvent> {
//         // map(take_bit(5usize), |what| SvcEvent {
//         //     event_count: what,
//         //     rest: 0u8
//         // })(i)
//         map(
//             tuple((bits(take_n_bit(5)), take(1usize))),
//             |(event_count, events)| SvcEvent {
//                 event_count
//                 // TODO
//                 rest: rest[0],
//             },
//         )(i)
//         // let p1 = bits(take_bit(5usize));
//         // let p2 = map(p1, |event_count| SvcEvent { event_count });

//         // let (i, event_count) = bits(take_5_bit)(i)?;
//     }

//     fn write(i: SvcEvent) -> Vec<u8> {
//         // let mut writer: ByteWriter::new();
//     }
// }
