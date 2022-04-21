use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::misc::Instance, errors::Error> {
    let mut item_pid_bytes = [0u8; 4];
    match source.read_exact(&mut item_pid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let item_pid = u32::from_be_bytes(item_pid_bytes);

    let mut caliber_bytes = [0u8; 4];
    match source.read_exact(&mut caliber_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let caliber_raw = u32::from_be_bytes(caliber_bytes);

    let caliber =
        match caliber_raw {
            0 => None,
            value => Some(
                match object::common::weapons::Caliber::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let mut count_bytes = [0u8; 4];
    match source.read_exact(&mut count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let count = u32::from_be_bytes(count_bytes);

    Ok(object::item::misc::Instance {
        count,
        caliber,
        connections: object::item::misc::Connections {
            power_item_id: item_pid
        },
    })
}