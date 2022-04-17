use super::*;

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

use std::ops::Range;
use std::collections::{HashMap, HashSet};

use super::super::super::common::types::ScaledValue;

pub mod errors {
    #[derive(Debug)]
    pub enum Format {
        Type,
        Flags,
        Data,
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
        connections: meta::info::Connections {
            sprite_id,
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

                    let mut armor_male_fid_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_male_fid_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_male_fid =
                        u32::from_be_bytes(match armor_male_fid_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

                    let mut armor_female_fid_bytes = vec![0u8; size_of::<u32>()];
                    match source.read_exact(&mut armor_female_fid_bytes) {
                        Err(error) => return Err(errors::Error::Read(error)),
                        Ok(value) => value,
                    };

                    let armor_female_fid =
                        u32::from_be_bytes(match armor_female_fid_bytes.try_into() {
                            Err(_) => return Err(errors::Error::Source),
                            Ok(value) => value,
                        });

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
                                sprite_ids: HashMap::from([
                                    (object::common::critter::Gender::Male, armor_male_fid),
                                    (object::common::critter::Gender::Female, armor_female_fid)
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
                // 2 => {}
                // 3 => {}
                // 4 => {}
                // 5 => {}
                // 6 => {}
                _ => return Err(errors::Error::Format(errors::Format::Type)),
            };

            object::Type::Item(
                object::item::Instance {
                    r#type: item_type,
                    is_hidden: item_is_hidden,
                    flags: item_flags,
                    actions: item_actions,
                    material: match object::common::world::Material::try_from(material_id) {
                        Ok(value) => value,
                        Err(_) => return Err(errors::Error::Format(errors::Format::Data))
                    },
                    cost: item_cost,
                    size: item_size,
                    weight: item_weight,
                    connections: object::item::Connections {
                        sprite_id: item_sprite_id,
                        script_id: match script_id {
                            0xFFFFFFFF => None,
                            value => Some(value)
                        },
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