use super::*;

pub struct ServerInfo {}
impl<'a> NetMsgDoer<'a, SvcServerInfo<'a>> for ServerInfo {
    fn parse(i: &'a [u8]) -> IResult<&[u8], SvcServerInfo> {
        map(
            tuple((
                le_i32,
                le_i32,
                le_i32,
                count(le_u8, 16),
                le_u8,
                le_u8,
                le_u8,
                null_string,
                null_string,
                null_string,
                null_string,
                le_u8,
            )),
            |(
                protocol,
                spawn_count,
                map_checksum,
                client_dll_hash,
                max_players,
                player_index,
                is_deathmatch,
                game_dir,
                hostname,
                map_file_name,
                map_cycle,
                unknown,
            )| SvcServerInfo {
                protocol,
                spawn_count,
                map_checksum,
                client_dll_hash,
                max_players,
                player_index,
                is_deathmatch,
                game_dir,
                hostname,
                map_file_name,
                map_cycle,
                unknown,
            },
        )(i)
    }

    fn write(i: SvcServerInfo) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(11u8);

        writer.append_i32(i.protocol);
        writer.append_i32(i.spawn_count);
        writer.append_i32(i.map_checksum);
        writer.append_u8_slice(&i.client_dll_hash);
        writer.append_u8(i.max_players);
        writer.append_u8(i.player_index);
        writer.append_u8(i.is_deathmatch);
        writer.append_u8_slice(i.game_dir);
        writer.append_u8_slice(i.hostname);
        writer.append_u8_slice(i.map_file_name);
        writer.append_u8_slice(i.map_cycle);
        writer.append_u8(i.unknown);

        writer.data
    }
}
