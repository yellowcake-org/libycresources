use super::*;

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

use std::ops::Range;
use std::time::Duration;
use std::collections::{HashMap, HashSet};

use super::super::super::common::types::ScaledValue;

pub mod errors {
    #[derive(Debug)]
    pub enum Format {
        Type,
        Data,
        Flags,
        Consistency,
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

    let object_id_bytes = &id_bytes[(size_of::<u32>() - size_of::<u16>())..size_of::<u32>()];
    let object_id = u16::from_be_bytes(match object_id_bytes.try_into() {
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

    let mut sprite_id_bytes = [0u8; 4];
    match source.read_exact(&mut sprite_id_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut lradius_bytes = vec![0u8; size_of::<u32>()];
    match source.read_exact(&mut lradius_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let lradius = u32::from_be_bytes(match lradius_bytes.try_into() {
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

    let meta = meta::Info {
        light: meta::info::Light {
            distance: ScaledValue {
                value: match u8::try_from(lradius) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                },
                scale: Range { start: 0, end: 8 },
            },
            intensity: ScaledValue {
                value: match u16::try_from(lintensity) {
                    Ok(value) => value,
                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                },
                scale: Range { start: 0, end: u16::MAX },
            },
        },
        flags: flagset,
        sprite: match object::common::sprite::Instance::try_from(sprite_id_bytes) {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
        },
        connections: meta::info::Connections {
            description_id: text_id,
        },
    };

    let r#type = id_bytes[0];
    let object_type = match r#type {
        0 => {
            let mut item_flags_bytes = vec![0u8; 3];
            match source.read_exact(&mut item_flags_bytes[1..=3]) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let mut item_flags: HashSet<object::item::Flag> = HashSet::new();
            let mut weapon_flags: HashSet<object::item::weapon::Flag> = HashSet::new();
            let mut item_actions: HashSet<object::common::actions::Instance> = HashSet::new();

            if (item_flags_bytes[0] & 0x01) == 0x01 {
                if !weapon_flags.insert(object::item::weapon::Flag::BigGun) {
                    return Err(errors::Error::Format(errors::Format::Flags));
                }
            }

            if (item_flags_bytes[0] & 0x02) == 0x02 {
                if !weapon_flags.insert(object::item::weapon::Flag::SecondHand) {
                    return Err(errors::Error::Format(errors::Format::Flags));
                }
            }

            if (item_flags_bytes[0] & 0x80) == 0x80 {
                if !item_actions.insert(object::common::actions::Instance::PickUp) {
                    return Err(errors::Error::Format(errors::Format::Flags));
                }
            }

            if (item_flags_bytes[2] & 0x08) == 0x08 {
                if !item_flags.insert(object::item::Flag::Hidden) {
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

            let attack_mode_primary_raw = attack_modes & 0xf;
            let attack_mode_secondary_raw = (attack_modes >> 4) & 0xf;

            let attack_mode_primary =
                match attack_mode_primary_raw {
                    0 => None,
                    value => Some(
                        match object::item::weapon::attack::Mode::try_from(value) {
                            Ok(value) => value,
                            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                        }
                    )
                };

            let attack_mode_secondary =
                match attack_mode_secondary_raw {
                    0 => None,
                    value => Some(
                        match object::item::weapon::attack::Mode::try_from(value) {
                            Ok(value) => value,
                            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                        }
                    )
                };

            let mut item_script_id_bytes = [0u8; 4];
            match source.read_exact(&mut item_script_id_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

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

            let item_size = u32::from_be_bytes(match item_size_bytes.try_into() {
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

            let mut item_sprite_id_bytes = [0u8; 4];
            match source.read_exact(&mut item_sprite_id_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let mut item_sound_ids_bytes = vec![0u8; size_of::<u8>()];
            match source.read_exact(&mut item_sound_ids_bytes) {
                Err(error) => return Err(errors::Error::Read(error)),
                Ok(value) => value,
            };

            let item_sound_ids = u8::from_be_bytes(
                match item_sound_ids_bytes.try_into() {
                    Err(_) => return Err(errors::Error::Source),
                    Ok(value) => value,
                }
            );

            let item_type = match item_type {
                0 => {
                    let mut armor_ac_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_ac_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_ac =
                        u32::from_be_bytes(match armor_ac_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_normal_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_normal_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_normal =
                        u32::from_be_bytes(match armor_dr_normal_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_laser_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_laser_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_laser =
                        u32::from_be_bytes(match armor_dr_laser_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_fire_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_fire_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_fire =
                        u32::from_be_bytes(match armor_dr_fire_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_plasma_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_plasma_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_plasma =
                        u32::from_be_bytes(match armor_dr_plasma_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_electrical_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_electrical_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_electrical =
                        u32::from_be_bytes(match armor_dr_electrical_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_emp_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_emp_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_emp =
                        u32::from_be_bytes(match armor_dr_emp_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dr_explosive_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dr_explosive_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dr_explosive =
                        u32::from_be_bytes(match armor_dr_explosive_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_normal_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_normal_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_normal =
                        u32::from_be_bytes(match armor_dt_normal_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_laser_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_laser_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_laser =
                        u32::from_be_bytes(match armor_dt_laser_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_fire_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_fire_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_fire =
                        u32::from_be_bytes(match armor_dt_fire_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_plasma_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_plasma_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_plasma =
                        u32::from_be_bytes(match armor_dt_plasma_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_electrical_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_electrical_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_electrical =
                        u32::from_be_bytes(match armor_dt_electrical_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_emp_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_emp_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_emp =
                        u32::from_be_bytes(match armor_dt_emp_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_dt_explosive_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_dt_explosive_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_dt_explosive =
                        u32::from_be_bytes(match armor_dt_explosive_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_perk_bytes = vec![0u8; size_of::<i32>()];
                    match source.read_exact(&mut armor_perk_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_perk =
                        i32::from_be_bytes(match armor_perk_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_male_fid_bytes = [0u8; 4];
                    match source.read_exact(&mut armor_male_fid_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let mut armor_female_fid_bytes = [0u8; 4];
                    match source.read_exact(&mut armor_female_fid_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    object::item::Type::Armor(
                        object::item::armor::Instance {
                            class: armor_ac,
                            threshold: HashMap::from([
                                (object::common::combat::damage::Type::Default, armor_dt_normal),
                                (object::common::combat::damage::Type::Laser, armor_dt_laser),
                                (object::common::combat::damage::Type::Fire, armor_dt_fire),
                                (object::common::combat::damage::Type::Plasma, armor_dt_plasma),
                                (object::common::combat::damage::Type::Electrical, armor_dt_electrical),
                                (object::common::combat::damage::Type::Emp, armor_dt_emp),
                                (object::common::combat::damage::Type::Explosive, armor_dt_explosive),
                            ]),
                            resistance: HashMap::from([
                                (object::common::combat::damage::Type::Default, armor_dr_normal),
                                (object::common::combat::damage::Type::Laser, armor_dr_laser),
                                (object::common::combat::damage::Type::Fire, armor_dr_fire),
                                (object::common::combat::damage::Type::Plasma, armor_dr_plasma),
                                (object::common::combat::damage::Type::Electrical, armor_dr_electrical),
                                (object::common::combat::damage::Type::Emp, armor_dr_emp),
                                (object::common::combat::damage::Type::Explosive, armor_dr_explosive),
                            ]),
                            perk: match armor_perk {
                                -1 => Option::None,
                                value => Option::Some(
                                    match object::common::critter::Perk::try_from(value) {
                                        Ok(value) => value,
                                        Err(_) =>
                                            return Err(errors::Error::Format(errors::Format::Data))
                                    }
                                ),
                            },
                            appearance: object::item::armor::Appearance {
                                sprites: HashMap::from([
                                    (object::common::critter::Gender::Male,
                                     match object::common::sprite::Instance::try_from(armor_male_fid_bytes) {
                                         Ok(value) => value,
                                         Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                                     }),
                                    (object::common::critter::Gender::Female,
                                     match object::common::sprite::Instance::try_from(armor_female_fid_bytes) {
                                         Ok(value) => value,
                                         Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                                     })
                                ])
                            },
                        }
                    )
                }
                1 => {
                    let mut container_size_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut container_size_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let container_size =
                        u32::from_be_bytes(match container_size_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut container_flags_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut container_flags_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let mut container_flags: HashSet<object::item::container::Flag> =
                        HashSet::new();

                    if (container_flags_bytes[3] & 0x01) == 0x01 {
                        if !container_flags.insert(object::item::container::Flag::NoPickUp) {
                            return Err(errors::Error::Format(errors::Format::Flags));
                        }
                    }

                    if (container_flags_bytes[3] & 0x08) == 0x08 {
                        if !container_flags.insert(object::item::container::Flag::MagicHands) {
                            return Err(errors::Error::Format(errors::Format::Flags));
                        }
                    }

                    object::item::Type::Container(
                        object::item::container::Instance {
                            size: container_size,
                            flags: container_flags,
                        }
                    )
                }
                2 => {
                    let mut drug_stat0_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_stat0_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_stat0_raw =
                        i32::from_be_bytes(match drug_stat0_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_stat1_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_stat1_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_stat1_raw =
                        i32::from_be_bytes(match drug_stat1_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_stat2_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_stat2_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_stat2_raw =
                        i32::from_be_bytes(match drug_stat2_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect0_amount0_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect0_amount0_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect0_amount0_raw =
                        u32::from_be_bytes(match drug_effect0_amount0_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect0_amount1_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect0_amount1_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect0_amount1_raw =
                        u32::from_be_bytes(match drug_effect0_amount1_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect0_amount2_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect0_amount2_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect0_amount2_raw =
                        u32::from_be_bytes(match drug_effect0_amount2_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect1_duration_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect1_duration_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect1_duration_raw =
                        u32::from_be_bytes(match drug_effect1_duration_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect1_amount0_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect1_amount0_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect1_amount0_raw =
                        u32::from_be_bytes(match drug_effect1_amount0_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect1_amount1_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect1_amount1_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect1_amount1_raw =
                        u32::from_be_bytes(match drug_effect1_amount1_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect1_amount2_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect1_amount2_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect1_amount2_raw =
                        u32::from_be_bytes(match drug_effect1_amount2_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect2_duration_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect2_duration_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect2_duration_raw =
                        u32::from_be_bytes(match drug_effect2_duration_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect2_amount0_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect2_amount0_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect2_amount0_raw =
                        u32::from_be_bytes(match drug_effect2_amount0_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect2_amount1_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect2_amount1_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect2_amount1_raw =
                        u32::from_be_bytes(match drug_effect2_amount1_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_effect2_amount2_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_effect2_amount2_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_effect2_amount2_raw =
                        u32::from_be_bytes(match drug_effect2_amount2_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_addiction_rate_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_addiction_rate_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_addiction_rate_raw =
                        u32::from_be_bytes(match drug_addiction_rate_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_addiction_perk_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_addiction_perk_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_addiction_perk_raw =
                        i32::from_be_bytes(match drug_addiction_perk_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut drug_addiction_delay_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut drug_addiction_delay_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let drug_addiction_delay_raw =
                        u32::from_be_bytes(match drug_addiction_delay_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let statistic0 =
                        match drug_stat0_raw > 0 {
                            false => Option::None,
                            true => Option::Some(
                                match
                                object::common::critter::Statistic::try_from(drug_stat0_raw) {
                                    Ok(value) => value,
                                    Err(_) =>
                                        return Err(errors::Error::Format(errors::Format::Data))
                                }
                            )
                        };

                    let statistic1 =
                        match object::common::critter::Statistic::try_from(drug_stat1_raw) {
                            Ok(value) => value,
                            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                        };

                    let statistic2 =
                        match object::common::critter::Statistic::try_from(drug_stat2_raw) {
                            Ok(value) => value,
                            Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                        };

                    let mut impact0 = HashMap::from([
                        (statistic1.clone(), match statistic0 {
                            Some(_) => object::item::drug::Amount::Fixed(drug_effect0_amount1_raw),
                            None => object::item::drug::Amount::Random(
                                drug_effect0_amount0_raw..=drug_effect0_amount1_raw
                            ),
                        }),
                        (statistic2.clone(), object::item::drug::Amount::Fixed(drug_effect0_amount2_raw))
                    ]);

                    if let Some(value) = statistic0.clone() {
                        impact0.entry(value.clone()).or_insert(
                            object::item::drug::Amount::Fixed(
                                drug_effect0_amount0_raw
                            ));
                    }

                    let effect0 = object::item::drug::Effect { delay: None, impact: impact0 };

                    let mut impact1 = HashMap::from([
                        (statistic1.clone(), match statistic0 {
                            Some(_) => object::item::drug::Amount::Fixed(drug_effect1_amount1_raw),
                            None => object::item::drug::Amount::Random(
                                drug_effect1_amount0_raw..=drug_effect1_amount1_raw
                            ),
                        }),
                        (statistic2.clone(), object::item::drug::Amount::Fixed(drug_effect1_amount2_raw))
                    ]);

                    if let Some(value) = statistic0.clone() {
                        impact1.entry(value.clone()).or_insert(
                            object::item::drug::Amount::Fixed(drug_effect1_amount0_raw)
                        );
                    }

                    let effect1 = object::item::drug::Effect {
                        delay: Some(Duration::new(drug_effect1_duration_raw as u64 * 60, 0)),
                        impact: impact1,
                    };

                    let mut impact2 = HashMap::from([
                        (statistic1.clone(), match statistic0 {
                            Some(_) => object::item::drug::Amount::Fixed(drug_effect2_amount1_raw),
                            None => object::item::drug::Amount::Random(
                                drug_effect2_amount0_raw..=drug_effect2_amount1_raw
                            ),
                        }),
                        (statistic2.clone(), object::item::drug::Amount::Fixed(drug_effect2_amount2_raw))
                    ]);

                    if let Some(value) = statistic0.clone() {
                        impact2.entry(value.clone()).or_insert(
                            object::item::drug::Amount::Fixed(drug_effect2_amount0_raw)
                        );
                    }

                    let effect2 = object::item::drug::Effect {
                        delay: Some(Duration::new(drug_effect2_duration_raw as u64 * 60, 0)),
                        impact: impact2,
                    };

                    if drug_effect2_duration_raw <= drug_effect1_duration_raw {
                        return Err(errors::Error::Format(errors::Format::Consistency));
                    }

                    let addiction = object::item::drug::Addiction {
                        perk: match drug_addiction_perk_raw {
                            -1 => None,
                            value => Some(
                                match object::common::critter::Perk::
                                try_from(value) {
                                    Ok(value) => value,
                                    Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                                }
                            )
                        },
                        delay: Duration::new(drug_addiction_delay_raw as u64 * 60, 0),
                        chance: ScaledValue { value: drug_addiction_rate_raw as u8, scale: 0u8..101u8 },
                    };

                    object::item::Type::Drug(
                        object::item::drug::Instance {
                            effects: [effect0, effect1, effect2],
                            addiction,
                        }
                    )
                }
                3 => {
                    let mut weapon_animation_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_animation_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_animation_raw = u32::from_be_bytes(
                        match weapon_animation_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let weapon_animation_code = (weapon_animation_raw & 0x000f) as u8;
                    let weapon_animation = match weapon_animation_code {
                        0x0 => None,
                        value => Some(
                            match object::item::weapon::Animation::try_from(value) {
                                Err(_) => return Err(errors::Error::Source),
                                Ok(value) => value,
                            }
                        )
                    };

                    let mut weapon_min_dmg_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_min_dmg_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_min_dmg = u32::from_be_bytes(
                        match weapon_min_dmg_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_max_dmg_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_max_dmg_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_max_dmg = u32::from_be_bytes(
                        match weapon_max_dmg_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_dmg_type_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_dmg_type_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_dmg_type_raw = u32::from_be_bytes(
                        match weapon_dmg_type_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let weapon_dmg_type = match object::common::combat::damage::Type::try_from(
                        weapon_dmg_type_raw as u8
                    ) {
                        Ok(value) => value,
                        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                    };

                    let weapon_damage = object::item::weapon::Damage {
                        value: weapon_min_dmg..=weapon_max_dmg,
                        r#type: weapon_dmg_type,
                    };

                    let mut weapon_dmg_range_max1_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_dmg_range_max1_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_dmg_range_max1 = u32::from_be_bytes(
                        match weapon_dmg_range_max1_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_dmg_range_max2_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_dmg_range_max2_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_dmg_range_max2 = u32::from_be_bytes(
                        match weapon_dmg_range_max2_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_projectile_header_bytes = vec![0u8; size_of::<u16>()];
                    match source.read_exact(&mut weapon_projectile_header_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_projectile_header = u16::from_be_bytes(
                        match weapon_projectile_header_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    if 0x0500 != weapon_projectile_header {
                        return Err(errors::Error::Format(errors::Format::Consistency));
                    }

                    let mut weapon_projectile_idx_bytes = vec![0u8; size_of::<u16>()];
                    match source.read_exact(&mut weapon_projectile_idx_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_projectile_idx = u16::from_be_bytes(
                        match weapon_projectile_idx_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_min_strength_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_min_strength_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_min_strength = u32::from_be_bytes(
                        match weapon_min_strength_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_cost1_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_cost1_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_cost1 = u32::from_be_bytes(
                        match weapon_cost1_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_cost2_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_cost2_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_cost2 = u32::from_be_bytes(
                        match weapon_cost2_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let weapon_attack1 = object::item::weapon::attack::Instance {
                        cost: weapon_cost1,
                        mode: attack_mode_primary,
                        range: 0..=weapon_dmg_range_max1,
                    };

                    let weapon_attack2 = object::item::weapon::attack::Instance {
                        cost: weapon_cost2,
                        mode: attack_mode_secondary,
                        range: 0..=weapon_dmg_range_max2,
                    };

                    let mut weapon_crit_list_idx_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_crit_list_idx_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_crit_list_idx = u32::from_be_bytes(
                        match weapon_crit_list_idx_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut weapon_perk_bytes = vec![0u8; size_of::<i32>()];
                    match source.read_exact(&mut weapon_perk_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_perk_raw =
                        i32::from_be_bytes(match weapon_perk_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let weapon_perk = match weapon_perk_raw {
                        -1 => Option::None,
                        value => Option::Some(
                            match object::common::critter::Perk::try_from(value) {
                                Ok(value) => value,
                                Err(_) =>
                                    return Err(errors::Error::Format(errors::Format::Data))
                            }
                        ),
                    };

                    let mut weapon_burst_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_burst_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_burst_count =
                        u32::from_be_bytes(match weapon_burst_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut weapon_caliber_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_caliber_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_caliber_raw =
                        u32::from_be_bytes(match weapon_caliber_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let weapon_caliber =
                        match weapon_caliber_raw {
                            0 => None,
                            value => Some(
                                match object::common::weapons::Caliber::try_from(value) {
                                    Ok(value) => value,
                                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                                }
                            )
                        };

                    let mut weapon_ammo_pid_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_ammo_pid_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_ammo_pid =
                        u32::from_be_bytes(match weapon_ammo_pid_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut weapon_capacity_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut weapon_capacity_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_capacity =
                        u32::from_be_bytes(match weapon_capacity_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut weapon_sound_ids_bytes = vec![0u8; size_of::<u8>()];
                    match source.read_exact(&mut weapon_sound_ids_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let weapon_sound_ids = u8::from_be_bytes(
                        match weapon_sound_ids_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    object::item::Type::Weapon(object::item::weapon::Instance {
                        flags: weapon_flags,
                        damage: weapon_damage,
                        attacks: [weapon_attack1, weapon_attack2],
                        animation: weapon_animation,
                        requirements: object::item::weapon::Requirements {
                            strength: weapon_min_strength
                        },
                        rounds: object::item::weapon::Rounds {
                            burst: weapon_burst_count,
                            magazine: weapon_capacity,
                        },
                        caliber: weapon_caliber,
                        perk: weapon_perk,
                        connections: object::item::weapon::Connections {
                            ammo_item_id: weapon_ammo_pid,
                            failure_list_id: weapon_crit_list_idx,
                            projectile_misc_id: weapon_projectile_idx,
                            _sounds_ids: weapon_sound_ids,
                        },
                    })
                }
                4 => {
                    let mut ammo_caliber_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut ammo_caliber_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let ammo_caliber_raw = u32::from_be_bytes(
                        match ammo_caliber_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let ammo_caliber =
                        match ammo_caliber_raw {
                            0 => None,
                            value => Some(
                                match object::common::weapons::Caliber::try_from(value) {
                                    Ok(value) => value,
                                    Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                                }
                            )
                        };

                    let mut ammo_count_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut ammo_count_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let ammo_count = u32::from_be_bytes(
                        match ammo_count_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut ammo_ac_modifier_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut ammo_ac_modifier_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let ammo_ac_modifier = u32::from_be_bytes(
                        match ammo_ac_modifier_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut ammo_dr_modifier_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut ammo_dr_modifier_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let ammo_dr_modifier = u32::from_be_bytes(
                        match ammo_dr_modifier_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut ammo_dmg_multiplier_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut ammo_dmg_multiplier_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let ammo_dmg_multiplier = u32::from_be_bytes(
                        match ammo_dmg_multiplier_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut ammo_dmg_divider_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut ammo_dmg_divider_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let ammo_dmg_divider = u32::from_be_bytes(
                        match ammo_dmg_divider_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    object::item::Type::Ammo(
                        object::item::ammo::Instance {
                            count: ammo_count,
                            caliber: ammo_caliber,
                            adjustments: object::item::ammo::adjustments::Instance {
                                armor: object::item::ammo::adjustments::Armor {
                                    class: ammo_ac_modifier,
                                    resistance: ammo_dr_modifier,
                                },
                                damage: object::item::ammo::adjustments::Damage {
                                    divider: ammo_dmg_divider,
                                    multiplier: ammo_dmg_multiplier,
                                },
                            },
                        }
                    )
                }
                5 => {
                    let mut misc_item_pid_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut misc_item_pid_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let misc_item_pid = u32::from_be_bytes(
                        match misc_item_pid_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    let mut misc_caliber_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut misc_caliber_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let misc_caliber_raw = u32::from_be_bytes(
                        match misc_caliber_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

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

                    let mut misc_count_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut misc_count_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let misc_count = u32::from_be_bytes(
                        match misc_count_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    object::item::Type::Misc(
                        object::item::misc::Instance {
                            count: misc_count,
                            caliber: misc_caliber,
                            connections: object::item::misc::Connections {
                                power_item_id: misc_item_pid
                            },
                        }
                    )
                }
                6 => {
                    let mut key_code_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut key_code_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let key_code = u32::from_be_bytes(
                        match key_code_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        }
                    );

                    object::item::Type::Key(
                        object::item::key::Instance {
                            code: key_code
                        }
                    )
                }
                _ => return Err(errors::Error::Format(errors::Format::Type)),
            };

            object::Type::Item(
                object::item::Instance {
                    r#type: item_type,
                    flags: item_flags,
                    sprite: match object::common::sprite::Instance::try_from(item_sprite_id_bytes) {
                        Ok(value) => value,
                        Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                    },
                    script: match item_script_id_bytes {
                        [0xFF, 0xFF, 0xFF, 0xFF] => None,
                        value => match object::common::script::Instance::try_from(value) {
                            Ok(value) => Some(value),
                            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                        }
                    },
                    actions: item_actions,
                    material: match object::common::world::Material::try_from(material_id) {
                        Ok(value) => value,
                        Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                    },
                    size: item_size,
                    price: item_cost,
                    weight: item_weight,
                    connections: object::item::Connections {
                        _sounds_ids: item_sound_ids,
                    },
                }
            )
        }
        // 1 => {}
        // 2 => {}
        // 3 => {}
        // 4 => {}
        // 5 => {}
        _ => return Err(errors::Error::Format(errors::Format::Type)),
    };

    Ok(Prototype {
        id: object_id,
        meta,
        r#type: object_type,
    })
}

impl TryFrom<u32> for object::common::world::Material {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Glass),
            1 => Ok(Self::Metal),
            2 => Ok(Self::Plastic),
            3 => Ok(Self::Wood),
            4 => Ok(Self::Dirt),
            5 => Ok(Self::Stone),
            6 => Ok(Self::Cement),
            7 => Ok(Self::Leather),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u8> for object::item::weapon::attack::Mode {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Punch),
            2 => Ok(Self::Kick),
            3 => Ok(Self::Swing),
            4 => Ok(Self::Thrust),
            5 => Ok(Self::Throw),
            6 => Ok(Self::FireSingle),
            7 => Ok(Self::FireBurst),
            8 => Ok(Self::Flame),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<i32> for object::common::critter::Perk {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Awareness),
            2 => Ok(Self::BonusHtHAttacks),
            3 => Ok(Self::BonusHtHDamage),
            4 => Ok(Self::BonusMove),
            5 => Ok(Self::BonusRangedDamage),
            6 => Ok(Self::BonusRateOfFire),
            7 => Ok(Self::EarlierSequence),
            8 => Ok(Self::FasterHealing),
            9 => Ok(Self::MoreCriticals),
            10 => Ok(Self::NightVision),
            11 => Ok(Self::Presence),
            12 => Ok(Self::RadResistance),
            13 => Ok(Self::Toughness),
            14 => Ok(Self::StrongBack),
            15 => Ok(Self::Sharpshooter),
            16 => Ok(Self::SilentRunning),
            17 => Ok(Self::Survivalist),
            18 => Ok(Self::MasterTrader),
            19 => Ok(Self::Educated),
            20 => Ok(Self::Healer),
            21 => Ok(Self::FortuneFinder),
            22 => Ok(Self::BetterCriticals),
            23 => Ok(Self::Empathy),
            24 => Ok(Self::Slayer),
            25 => Ok(Self::Sniper),
            26 => Ok(Self::SilentDeath),
            27 => Ok(Self::ActionBoy),
            28 => Ok(Self::MentalBlock),
            29 => Ok(Self::Lifegiver),
            30 => Ok(Self::Dodger),
            31 => Ok(Self::Snakeater),
            32 => Ok(Self::MrFixit),
            33 => Ok(Self::Medic),
            34 => Ok(Self::MasterThief),
            35 => Ok(Self::Speaker),
            36 => Ok(Self::HeaveHo),
            37 => Ok(Self::FriendlyFoe),
            38 => Ok(Self::Pickpocket),
            39 => Ok(Self::Ghost),
            40 => Ok(Self::CultOfPersonality),
            41 => Ok(Self::Scrounger),
            42 => Ok(Self::Explorer),
            43 => Ok(Self::FlowerChild),
            44 => Ok(Self::Pathfinder),
            45 => Ok(Self::AnimalFriend),
            46 => Ok(Self::Scout),
            47 => Ok(Self::MysteriousStranger),
            48 => Ok(Self::Ranger),
            49 => Ok(Self::QuickPockets),
            50 => Ok(Self::SmoothTalker),
            51 => Ok(Self::SwiftLearner),
            52 => Ok(Self::Tag),
            53 => Ok(Self::Mutate),
            54 => Ok(Self::NukaColaAddiction),
            55 => Ok(Self::BuffoutAddiction),
            56 => Ok(Self::MentatsAddiction),
            57 => Ok(Self::PsychoAddiction),
            58 => Ok(Self::RadawayAddiction),
            59 => Ok(Self::WeaponLongRange),
            60 => Ok(Self::WeaponAccurate),
            61 => Ok(Self::WeaponPenetrate),
            62 => Ok(Self::WeaponKnockback),
            63 => Ok(Self::PoweredArmor),
            64 => Ok(Self::CombatArmor),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<i32> for object::common::critter::Statistic {
    type Error = errors::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Strength),
            2 => Ok(Self::Perception),
            3 => Ok(Self::Endurance),
            4 => Ok(Self::Charisma),
            5 => Ok(Self::Intelligence),
            6 => Ok(Self::Agility),
            7 => Ok(Self::Luck),
            8 => Ok(Self::MaximumHitPoints),
            9 => Ok(Self::MaximumActionPoints),
            10 => Ok(Self::ArmorClass),
            11 => Ok(Self::UnarmedDamage),
            12 => Ok(Self::MeleeDamage),
            13 => Ok(Self::CarryWeight),
            14 => Ok(Self::Sequence),
            15 => Ok(Self::HealingRate),
            16 => Ok(Self::CriticalChance),
            17 => Ok(Self::BetterCriticals),
            18 => Ok(Self::DamageThreshold),
            19 => Ok(Self::DamageThresholdLaser),
            20 => Ok(Self::DamageThresholdFire),
            21 => Ok(Self::DamageThresholdPlasma),
            22 => Ok(Self::DamageThresholdElectrical),
            23 => Ok(Self::DamageThresholdEMP),
            24 => Ok(Self::DamageThresholdExplosion),
            25 => Ok(Self::DamageResistance),
            26 => Ok(Self::DamageResistanceLaser),
            27 => Ok(Self::DamageResistanceFire),
            28 => Ok(Self::DamageResistancePlasma),
            29 => Ok(Self::DamageResistanceElectrical),
            30 => Ok(Self::DamageResistanceEMP),
            31 => Ok(Self::DamageResistanceExplosion),
            32 => Ok(Self::RadiationResistance),
            33 => Ok(Self::PoisonResistance),
            34 => Ok(Self::Age),
            35 => Ok(Self::Gender),
            36 => Ok(Self::CurrentHitPoints),
            37 => Ok(Self::CurrentPoisonLevel),
            38 => Ok(Self::CurrentRadiationLevel),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u8> for object::item::weapon::Animation {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Self::Knife),
            0x2 => Ok(Self::Club),
            0x3 => Ok(Self::Sledgehammer),
            0x4 => Ok(Self::Spear),
            0x5 => Ok(Self::Pistol),
            0x6 => Ok(Self::SubmachineGun),
            0x7 => Ok(Self::Rifle),
            0x8 => Ok(Self::BigGun),
            0x9 => Ok(Self::Minigun),
            0xA => Ok(Self::RocketLauncher),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u8> for object::common::combat::damage::Type {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::Laser),
            2 => Ok(Self::Fire),
            3 => Ok(Self::Plasma),
            4 => Ok(Self::Electrical),
            5 => Ok(Self::Emp),
            6 => Ok(Self::Explosive),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<u32> for object::common::weapons::Caliber {
    type Error = errors::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Rocket),
            2 => Ok(Self::FlamethrowerFuel),
            3 => Ok(Self::CEnergyCell),
            4 => Ok(Self::DEnergyCell),
            5 => Ok(Self::Remington223),
            6 => Ok(Self::FiveMillimeter),
            7 => Ok(Self::SnW40),
            8 => Ok(Self::TenMillimiter),
            9 => Ok(Self::Magnum44),
            10 => Ok(Self::FourteenMillimeter),
            11 => Ok(Self::TwelveGauge),
            12 => Ok(Self::NineMillimeter),
            13 => Ok(Self::Bb),
            _ => Err(errors::Error::Format(errors::Format::Data))
        }
    }
}

impl TryFrom<[u8; 4]> for object::common::sprite::Instance {
    type Error = errors::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let r#type = match value[0] {
            0x00 => object::common::sprite::Type::Item,
            0x01 => object::common::sprite::Type::Critter,
            0x02 => object::common::sprite::Type::Scenery,
            0x03 => object::common::sprite::Type::Wall,
            0x04 => object::common::sprite::Type::Tile,
            0x05 => object::common::sprite::Type::Background,
            0x06 => object::common::sprite::Type::Interface,
            0x07 => object::common::sprite::Type::Inventory,
            _ => return Err(errors::Error::Format(errors::Format::Data)),
        };

        let id = u16::from_be_bytes(match (&value[2..4]).try_into() {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        });

        return Ok(Self {
            id,
            r#type,
        });
    }
}

impl TryFrom<[u8; 4]> for object::common::script::Instance {
    type Error = errors::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let r#type = match value[0] {
            0x01 => object::common::script::Type::Spatial,
            0x02 => object::common::script::Type::Item,
            0x03 => object::common::script::Type::Scenery,
            0x04 => object::common::script::Type::Critter,
            _ => return Err(errors::Error::Format(errors::Format::Data)),
        };

        let id = u16::from_be_bytes(match (&value[2..4]).try_into() {
            Ok(value) => value,
            Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
        });

        return Ok(Self {
            id,
            r#type,
        });
    }
}