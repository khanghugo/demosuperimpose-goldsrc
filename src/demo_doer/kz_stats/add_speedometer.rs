use super::*;

pub fn add_speedometer<'a>(
    prev: Option<&KzInfo<'a>>,
    curr: Option<&KzInfo<'a>>,
) -> Option<SvcTempEntity<'a>> {
    if prev.is_none() || curr.is_none() {
        return None;
    }

    let prev = prev.unwrap();
    let curr = curr.unwrap();

    // Lots of assumptions here like frametime will be non zero.

    let frametime = curr.frametime - prev.frametime;
    if frametime == 0. {
        return None;
    }

    let abs_displacement = (0..2)
        .map(|i| curr.origin[i] - prev.origin[i])
        .fold(0., |acc, num| num * num + acc)
        .sqrt();
    let speed = abs_displacement / frametime;

    let message = format!("{:.1}\0", speed);
    let message = message.leak().as_bytes(); // leak() solves everything very nice

    let text = TeTextMessage {
        channel: 4,
        // (0, 0) is top left
        x: 0.48f32.coord_conversion(),
        y: 0.75f32.coord_conversion(),
        effect: 0,
        text_color: &[255, 255, 255, 0],
        effect_color: &[255, 255, 255, 0],
        fade_in_time: 25,
        fade_out_time: 76,
        hold_time: 60,
        effect_time: None,
        message,
    };

    let temp_entity = SvcTempEntity {
        entity_type: 29,
        entity: TempEntityEntity::TeTextMessage(text),
    };

    Some(temp_entity)
}
