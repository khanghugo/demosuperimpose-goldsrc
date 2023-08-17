use super::{utils::BitSliceCast, *};

pub struct Pings {}
impl<'a> NetMsgDoer<'a, SvcPings> for Pings {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcPings> {
        let mut br = BitReader::new(i);
        let mut pings: Vec<PingS> = vec![];

        while br.read_1_bit() {
            pings.push(PingS {
                has_ping_data: true,
                player_id: Some(br.read_n_bit(8).to_u8()),
                ping: Some(br.read_n_bit(8).to_u8()),
                loss: Some(br.read_n_bit(8).to_u8()),
            })
        }

        // If we exit the loop, it means we already read the has_ping_data = false bit.

        // Last element.
        pings.push(PingS {
            has_ping_data: false,
            player_id: None,
            ping: None,
            loss: None,
        });

        // Don't forget
        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((i, SvcPings { pings }))
    }

    fn write(i: SvcPings) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcPings as u8);

        let mut bw = BitWriter::new();

        for ping in i.pings {
            if ping.has_ping_data {
                bw.append_bit(true);
                bw.append_u8(ping.player_id.unwrap());
                bw.append_u8(ping.ping.unwrap());
                bw.append_u8(ping.loss.unwrap());
            } else {
                bw.append_bit(false);
            }
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
