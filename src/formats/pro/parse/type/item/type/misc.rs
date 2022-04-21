use super::super::super::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::misc::Instance, errors::Error> {
    let mut misc_item_pid_bytes = [0u8; 4];
    match source.read_exact(&mut misc_item_pid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let misc_item_pid = u32::from_be_bytes(misc_item_pid_bytes);

    let mut misc_caliber_bytes = [0u8; 4];
    match source.read_exact(&mut misc_caliber_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let misc_caliber_raw = u32::from_be_bytes(misc_caliber_bytes);

    let misc_caliber =
        match misc_caliber_raw {
            0 => None,
            value => Some(
                match object::common::weapons::Caliber::try_from(value) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                }
            )
        };

    let mut misc_count_bytes = [0u8; 4];
    match source.read_exact(&mut misc_count_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let misc_count = u32::from_be_bytes(misc_count_bytes);

    Ok(object::item::misc::Instance {
        count: misc_count,
        caliber: misc_caliber,
        connections: object::item::misc::Connections {
            power_item_id: misc_item_pid
        },
    })
}