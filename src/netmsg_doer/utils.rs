use super::*;

/// Does not return null terminator in the slice. Remember to add back.
pub fn null_string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until("\x00"), tag("\x00"))(i)
}
