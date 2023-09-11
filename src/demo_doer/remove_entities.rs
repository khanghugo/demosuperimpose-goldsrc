use super::*;

/// Simply removes entities.
pub fn remove_entities(demo: &mut Demo, listed_entities: Vec<u16>) {
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for entry in &mut demo.directory.entries {
        for frame in &mut entry.frames {
            if let FrameData::NetMsg((_, data)) = &mut frame.data {
                let (_, mut messages) =
                    parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages).unwrap();

                let mut marked_msg = vec![];

                for (msg_idx, msg) in messages.iter_mut().enumerate() {
                    match msg {
                        Message::EngineMessage(what) => match what {
                            EngineMessage::SvcSpawnBaseline(baseline) => {
                                for i in (0..baseline.entities.len()).rev() {
                                    if listed_entities
                                        .contains(&baseline.entities[i].index.to_u16())
                                    {
                                        baseline.entities.remove(i);
                                    }
                                }
                            }
                            EngineMessage::SvcPacketEntities(packet) => {
                                for i in (0..packet.entity_states.len()).rev() {
                                    if listed_entities
                                        .contains(&packet.entity_states[i].entity_index)
                                    {
                                        // lazy option so there's no need to to arithmetic.
                                        packet.entity_states[i].delta.clear();
                                    }
                                }
                            }
                            EngineMessage::SvcDeltaPacketEntities(packet) => {
                                for i in (0..packet.entity_states.len()).rev() {
                                    if listed_entities
                                        .contains(&packet.entity_states[i].entity_index)
                                    {
                                        if packet.entity_states[i].delta.is_some() {
                                            packet.entity_states[i].delta.as_mut().unwrap().clear();
                                        }
                                    }
                                }
                            }
                            EngineMessage::SvcSound(sound) => {
                                if listed_entities.contains(&sound.entity_index.to_u16()) {
                                    marked_msg.push(msg_idx);
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }

                for i in marked_msg.iter().rev() {
                    messages.remove(*i);
                }

                let write = write_netmsg(messages, &mut delta_decoders, &custom_messages);

                data.msg = write.leak();
                // data.msg = &[]; // sanity check
            }
        }
    }
}
