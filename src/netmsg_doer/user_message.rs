use super::*;

pub struct UserMessage {}
impl<'a> UserMessageDoer<'a, NetMsgUserMessage<'a>> for UserMessage {
    fn parse(
        i: &'a [u8],
        id: u8,
        custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
    ) -> IResult<&'a [u8], NetMsgUserMessage<'a>> {
        let custom_message = custom_messages.get(&id);

        let is_set = custom_message.is_some();
        let is_size = custom_message.is_some() && custom_message.unwrap().size > -1; // equivalent to -1

        // let (i, data) = if is_set {
        //     if is_size {
        //         take(custom_message.unwrap().size as usize)(i)?
        //     } else {
        //         let (i, length) = le_u8(i)?;
        //         take(length as usize)(i)?
        //     }
        // } else {
        //     take(1usize)(i)?
        // };

        let (i, data) = if is_size {
            take(custom_message.unwrap().size as usize)(i)?
        } else {
            let (i, length) = le_u8(i)?;
            take(length as usize)(i)?
        };

        Ok((
            i,
            NetMsgUserMessage {
                id,
                name: if is_set {
                    custom_message.unwrap().name
                } else {
                    b"\0"
                },
                data,
            },
        ))
    }

    fn write(
        i: NetMsgUserMessage,
        custom_messages: &mut HashMap<u8, SvcNewUserMsg<'a>>,
    ) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(i.id);

        if let Some(message) = custom_messages.get(&i.id) {
            if message.size == -1 {
                writer.append_u8(i.data.len() as u8);
            }
        }

        writer.append_u8_slice(i.data);

        writer.data
    }
}
