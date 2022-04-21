use std::collections::HashMap;

use super::super::super::*;
use super::super::super::traits::*;

pub(crate) fn instance<S: Read>(source: &mut S) -> Result<object::item::armor::Instance, errors::Error> {
    let mut ac_bytes = [0u8; 4];
    match source.read_exact(&mut ac_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let ac = u32::from_be_bytes(ac_bytes);

    let mut dr_normal_bytes = [0u8; 4];
    match source.read_exact(&mut dr_normal_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_normal = u32::from_be_bytes(dr_normal_bytes);

    let mut dr_laser_bytes = [0u8; 4];
    match source.read_exact(&mut dr_laser_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_laser = u32::from_be_bytes(dr_laser_bytes);

    let mut dr_fire_bytes = [0u8; 4];
    match source.read_exact(&mut dr_fire_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_fire = u32::from_be_bytes(dr_fire_bytes);

    let mut dr_plasma_bytes = [0u8; 4];
    match source.read_exact(&mut dr_plasma_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_plasma = u32::from_be_bytes(dr_plasma_bytes);

    let mut dr_electrical_bytes = [0u8; 4];
    match source.read_exact(&mut dr_electrical_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_electrical = u32::from_be_bytes(dr_electrical_bytes);

    let mut dr_emp_bytes = [0u8; 4];
    match source.read_exact(&mut dr_emp_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_emp = u32::from_be_bytes(dr_emp_bytes);

    let mut dr_explosive_bytes = [0u8; 4];
    match source.read_exact(&mut dr_explosive_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dr_explosive = u32::from_be_bytes(dr_explosive_bytes);

    let mut dt_normal_bytes = [0u8; 4];
    match source.read_exact(&mut dt_normal_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_normal = u32::from_be_bytes(dt_normal_bytes);

    let mut dt_laser_bytes = [0u8; 4];
    match source.read_exact(&mut dt_laser_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_laser = u32::from_be_bytes(dt_laser_bytes);

    let mut dt_fire_bytes = [0u8; 4];
    match source.read_exact(&mut dt_fire_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_fire = u32::from_be_bytes(dt_fire_bytes);

    let mut dt_plasma_bytes = [0u8; 4];
    match source.read_exact(&mut dt_plasma_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_plasma = u32::from_be_bytes(dt_plasma_bytes);

    let mut dt_electrical_bytes = [0u8; 4];
    match source.read_exact(&mut dt_electrical_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_electrical = u32::from_be_bytes(dt_electrical_bytes);

    let mut dt_emp_bytes = [0u8; 4];
    match source.read_exact(&mut dt_emp_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_emp = u32::from_be_bytes(dt_emp_bytes);

    let mut dt_explosive_bytes = [0u8; 4];
    match source.read_exact(&mut dt_explosive_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let dt_explosive = u32::from_be_bytes(dt_explosive_bytes);

    let mut perk_bytes = [0u8; 4];
    match source.read_exact(&mut perk_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let perk_raw = i32::from_be_bytes(perk_bytes);

    let mut male_fid_bytes = [0u8; 4];
    match source.read_exact(&mut male_fid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    let mut female_fid_bytes = [0u8; 4];
    match source.read_exact(&mut female_fid_bytes) {
        Err(error) => return Err(errors::Error::Read(error)),
        Ok(value) => value,
    };

    Ok(object::item::armor::Instance {
        class: ac,
        threshold: HashMap::from([
            (object::common::combat::damage::Type::Default, dt_normal),
            (object::common::combat::damage::Type::Laser, dt_laser),
            (object::common::combat::damage::Type::Fire, dt_fire),
            (object::common::combat::damage::Type::Plasma, dt_plasma),
            (object::common::combat::damage::Type::Electrical, dt_electrical),
            (object::common::combat::damage::Type::Emp, dt_emp),
            (object::common::combat::damage::Type::Explosive, dt_explosive),
        ]),
        resistance: HashMap::from([
            (object::common::combat::damage::Type::Default, dr_normal),
            (object::common::combat::damage::Type::Laser, dr_laser),
            (object::common::combat::damage::Type::Fire, dr_fire),
            (object::common::combat::damage::Type::Plasma, dr_plasma),
            (object::common::combat::damage::Type::Electrical, dr_electrical),
            (object::common::combat::damage::Type::Emp, dr_emp),
            (object::common::combat::damage::Type::Explosive, dr_explosive),
        ]),
        perk: match object::common::critter::Perk::try_from_optional(perk_raw, -1) {
            Ok(value) => value,
            Err(_) =>
                return Err(errors::Error::Format(errors::Format::Data))
        },
        appearance: object::item::armor::Appearance {
            sprites: HashMap::from([
                (object::common::critter::Gender::Male,
                 match object::common::sprite::Reference::try_from(male_fid_bytes) {
                     Ok(value) => value,
                     Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                 }),
                (object::common::critter::Gender::Female,
                 match object::common::sprite::Reference::try_from(female_fid_bytes) {
                     Ok(value) => value,
                     Err(_) => return Err(errors::Error::Format(errors::Format::Data)),
                 })
            ])
        },
    })
}