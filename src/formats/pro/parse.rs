use super::*;

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

use std::collections::{HashMap, HashSet};

pub mod errors {
    #[derive(Debug)]
    pub enum Format {
        Type,
        Flags,
    }

    #[derive(Debug)]
    pub enum Error {
        Read(std::io::Error),
        Format(Format),
        Source,
    }
}

pub fn prototype<S: Read + Seek>(source: &mut S) -> Result<Prototype, errors::Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(errors::Error::Read(error));
    }

    let mut id_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let r#type = id_bytes[0];
    let id = u32::from_be_bytes(match id_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut text_id_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut text_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let text_id = u32::from_be_bytes(match text_id_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut sprite_id_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let sprite_id = u32::from_be_bytes(match sprite_id_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut lradius_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut lradius_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lradius_id = u32::from_be_bytes(match lradius_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut lintensity_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut lintensity_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lintensity = u32::from_be_bytes(match lintensity_bytes.try_into() {
        Err(_) => return Err(errors::Error::Source),
        Ok(value) => value,
    });

    let mut flags_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut flags_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut flagset: HashSet<meta::info::flags::Instance> = HashSet::new();

    if (flags_bytes[0] & 0x08) == 0x08 {
        if !flagset.insert(meta::info::flags::Instance::Flat) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[0] & 0x10) == 0x10 {
        if !flagset.insert(meta::info::flags::Instance::NotBlocking) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x08) == 0x08 {
        if !flagset.insert(meta::info::flags::Instance::MultiHex) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x10) == 0x10 {
        if !flagset.insert(meta::info::flags::Instance::NotBordered) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x20) == 0x20 {
        if !flagset.insert(meta::info::flags::Instance::LightThrough) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[3] & 0x80) == 0x80 {
        if !flagset.insert(meta::info::flags::Instance::ShotThrough) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    }

    if (flags_bytes[1] & 0x80) == 0x80 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(None)) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[1] & 0x40) == 0x40 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Red,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x01) == 0x01 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Wall,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x02) == 0x02 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Glass,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x04) == 0x04 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Steam,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else if (flags_bytes[2] & 0x08) == 0x08 {
        if !flagset.insert(meta::info::flags::Instance::Transparency(Some(
            meta::info::flags::Transparency::Energy,
        ))) {
            return Err(errors::Error::Format(errors::Format::Flags));
        }
    } else {
        return Err(errors::Error::Format(errors::Format::Flags));
    }

    match r#type {
        0 => {
            let mut item_flags_bytes = vec![0u8; 3];
            match source.read_exact(&mut item_flags_bytes[1..=3]) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_is_hidden = (item_flags_bytes[2] & 0x08) == 0x08;
            let mut item_flags: HashSet<object::common::weapons::Flag> = HashSet::new();
            let mut item_actions: HashSet<object::common::actions::Instance> = HashSet::new();

            if (item_flags_bytes[0] & 0x01) == 0x01 {
                if !item_flags.insert(object::common::weapons::Flag::BigGun) {
                    return Err(errors::Error::Format(errors::Format::Flags));
                }
            }

            if (item_flags_bytes[0] & 0x02) == 0x02 {
                if !item_flags.insert(object::common::weapons::Flag::SecondHand) {
                    return Err(errors::Error::Format(errors::Format::Flags));
                }
            }

            if (item_flags_bytes[0] & 0x80) == 0x80 {
                if !item_actions.insert(object::common::actions::Instance::PickUp) {
                    return Err(errors::Error::Format(errors::Format::Flags));
                }
            }

            let can_use = (item_flags_bytes[0] & 0x08) == 0x08;
            let can_use_on = (item_flags_bytes[0] & 0x10) == 0x10;

            let usage = object::common::actions::Usage {
                itself: can_use,
                something: can_use_on,
                knees_down: false,
            };

            if !item_actions.insert(object::common::actions::Instance::Usage(usage)) {
                return Err(errors::Error::Format(errors::Format::Flags));
            }

            let mut attack_modes_bytes = vec![0u8; size_of::<u8>()];
            match source.read_exact(&mut attack_modes_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let attack_modes = u8::from_be_bytes(match attack_modes_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let attack_mode_primary = attack_modes & 0xf;
            let attack_mode_secondary = (attack_modes >> 4) & 0xf;

            fn attack_mode(raw: &u8) -> Option<object::item::weapon::attack::Mode> {
                return match raw {
                    1 => Some(object::item::weapon::attack::Mode::Punch),
                    2 => Some(object::item::weapon::attack::Mode::Kick),
                    3 => Some(object::item::weapon::attack::Mode::Swing),
                    4 => Some(object::item::weapon::attack::Mode::Thrust),
                    5 => Some(object::item::weapon::attack::Mode::Throw),
                    6 => Some(object::item::weapon::attack::Mode::FireSingle),
                    7 => Some(object::item::weapon::attack::Mode::FireBurst),
                    8 => Some(object::item::weapon::attack::Mode::Flame),
                    _ => None,
                };
            }

            let mut script_id_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut script_id_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let script_id = u32::from_be_bytes(match script_id_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut item_type_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut item_type_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_type = u32::from_be_bytes(match item_type_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut material_id_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut material_id_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let material_id = u32::from_be_bytes(match material_id_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut item_size_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut item_size_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_size_bytes = u32::from_be_bytes(match item_size_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut item_weight_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut item_weight_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_weight = u32::from_be_bytes(match item_weight_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut item_cost_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut item_cost_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_cost = u32::from_be_bytes(match item_cost_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut item_sprite_id_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut item_sprite_id_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_sprite_id = u32::from_be_bytes(match item_sprite_id_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });

            let mut item_sound_ids_bytes = vec![0u8; size_of::<u32>()];
            match source.read_exact(&mut item_sound_ids_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_sound_ids = u32::from_be_bytes(match item_sound_ids_bytes.try_into() {
                Err(_) => return Err(errors::Error::Source),
                Ok(value) => value,
            });
        }
        1 => {}
        2 => {}
        3 => {}
        4 => {}
        5 => {}
        _ => return Err(errors::Error::Format(errors::Format::Type)),
    }

    Err(errors::Error::Source)
}
